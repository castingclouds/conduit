use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tower_http::cors::{Any, CorsLayer};
use tokio::sync::oneshot;
use tracing::{info, error};

use crate::memory::{Memory, MemoryStore};
use super::openai;
use super::state::ServerState;

pub async fn start_server(
    memory_store: Arc<MemoryStore>,
    addr: SocketAddr,
) -> Result<oneshot::Sender<()>, String> {
    info!("Starting API server on {}", addr);
    
    // Create a channel for shutdown signal
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    info!("Created shutdown channel");
    
    // Ensure the memory directory exists
    let base_path = memory_store.base_path.clone();
    info!("Memory base path: {:?}", base_path);
    
    if !base_path.exists() {
        info!("Memory directory does not exist, creating it");
        match std::fs::create_dir_all(&base_path) {
            Ok(_) => info!("Successfully created memory directory"),
            Err(e) => {
                error!("Failed to create memory directory: {:?}", e);
                return Err(format!("Failed to create memory directory: {}", e));
            }
        }
    } else {
        info!("Memory directory already exists");
    }
    
    // Create shared state
    info!("Creating shared server state");
    let state = Arc::new(ServerState {
        memory_store,
        shutdown_tx: Mutex::new(Some(shutdown_tx)),
    });
    info!("Server state created successfully");
    
    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Create router
    let app = Router::new()
        // Memory API routes
        .route("/api/memories", get(list_memories_handler).post(create_memory_handler))
        .route("/api/memories/:id", get(get_memory_handler).delete(delete_memory_handler))
        .route("/api/memories/search", post(search_memories_handler))
        
        // OpenAI-compatible API routes
        .nest("/v1", openai::router())
        
        // Add CORS and state
        .layer(cors)
        .with_state(state.clone());
    
    // Start the server
    info!("Starting API server on {}", addr);
    
    // Spawn the server task
    tokio::spawn(async move {
        info!("[SERVER] Binding TCP listener to {}", addr);
        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(l) => {
                info!("[SERVER] TCP listener bound successfully");
                l
            },
            Err(e) => {
                error!("[SERVER] Failed to bind TCP listener: {:?}", e);
                return;
            }
        };
        
        info!("[SERVER] Starting axum server");
        match axum::serve(listener, app)
            .with_graceful_shutdown(async {
                info!("[SERVER] Waiting for shutdown signal");
                match shutdown_rx.await {
                    Ok(_) => info!("[SERVER] Shutdown signal received"),
                    Err(e) => info!("[SERVER] Shutdown channel error: {:?}", e)
                }
                info!("[SERVER] API server shutting down");
            })
            .await
        {
            Ok(_) => info!("[SERVER] Server shut down gracefully"),
            Err(e) => error!("[SERVER] Server error: {:?}", e)
        }
        info!("[SERVER] Server task completed");
    });
    
    // Create a new shutdown sender that won't be dropped immediately
    let (new_shutdown_tx, _) = oneshot::channel::<()>();
    Ok(new_shutdown_tx)
}

async fn list_memories(
    State(state): State<Arc<ServerState>>,
) -> impl IntoResponse {
    info!("[SERVER] Handling list_memories request");
    
    // Ensure the memory directory exists
    let base_path = state.memory_store.base_path.clone();
    info!("[SERVER] Memory base path: {:?}", base_path);
    
    if !base_path.exists() {
        info!("[SERVER] Memory directory does not exist, creating it");
        match std::fs::create_dir_all(&base_path) {
            Ok(_) => info!("[SERVER] Successfully created memory directory"),
            Err(e) => {
                error!("[SERVER] Failed to create memory directory: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create memory directory: {}", e)).into_response();
            }
        }
    }
    
    info!("[SERVER] Calling memory_store.list()");
    match state.memory_store.list() {
        Ok(memories) => (StatusCode::OK, Json(memories)).into_response(),
        Err(err) => {
            error!("Error listing memories: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

async fn get_memory(
    State(state): State<Arc<ServerState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    info!("[SERVER] Handling get_memory request for id: {}", id);
    
    // Ensure the memory directory exists
    let base_path = state.memory_store.base_path.clone();
    info!("[SERVER] Memory base path: {:?}", base_path);
    
    if !base_path.exists() {
        info!("[SERVER] Memory directory does not exist, creating it");
        match std::fs::create_dir_all(&base_path) {
            Ok(_) => info!("[SERVER] Successfully created memory directory"),
            Err(e) => {
                error!("[SERVER] Failed to create memory directory: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create memory directory: {}", e)).into_response();
            }
        }
    }
    
    info!("[SERVER] Calling memory_store.get() for id: {}", id);
    match state.memory_store.get(&id) {
        Ok(memory) => (StatusCode::OK, Json(memory)).into_response(),
        Err(err) => {
            error!("Error getting memory {}: {:?}", id, err);
            let status = match err {
                crate::memory::MemoryError::NotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, err.to_string()).into_response()
        }
    }
}

#[derive(serde::Deserialize)]
struct CreateMemoryRequest {
    title: String,
    content: String,
    tags: Vec<String>,
}

async fn create_memory(
    State(state): State<Arc<ServerState>>,
    Json(req): Json<CreateMemoryRequest>,
) -> impl IntoResponse {
    info!("[SERVER] Handling create_memory request with title: {}", req.title);
    
    // Ensure the memory directory exists
    let base_path = state.memory_store.base_path.clone();
    info!("[SERVER] Memory base path: {:?}", base_path);
    
    if !base_path.exists() {
        info!("[SERVER] Memory directory does not exist, creating it");
        match std::fs::create_dir_all(&base_path) {
            Ok(_) => info!("[SERVER] Successfully created memory directory"),
            Err(e) => {
                error!("[SERVER] Failed to create memory directory: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create memory directory: {}", e)).into_response();
            }
        }
    }
    
    info!("[SERVER] Creating new memory with title: {}", req.title);
    let memory = Memory::new(req.title, req.content, req.tags);
    info!("[SERVER] Generated memory ID: {}", memory.id);
    
    info!("[SERVER] Calling memory_store.save()");
    match state.memory_store.save(&memory) {
        Ok(_) => (StatusCode::CREATED, Json(memory)).into_response(),
        Err(err) => {
            error!("Error creating memory: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

async fn delete_memory(
    State(state): State<Arc<ServerState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    info!("[SERVER] Handling delete_memory request for id: {}", id);
    
    // Ensure the memory directory exists
    let base_path = state.memory_store.base_path.clone();
    info!("[SERVER] Memory base path: {:?}", base_path);
    
    if !base_path.exists() {
        info!("[SERVER] Memory directory does not exist, creating it");
        match std::fs::create_dir_all(&base_path) {
            Ok(_) => info!("[SERVER] Successfully created memory directory"),
            Err(e) => {
                error!("[SERVER] Failed to create memory directory: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create memory directory: {}", e)).into_response();
            }
        }
    }
    
    info!("[SERVER] Calling memory_store.delete() for id: {}", id);
    match state.memory_store.delete(&id) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => {
            error!("Error deleting memory {}: {:?}", id, err);
            let status = match err {
                crate::memory::MemoryError::NotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, err.to_string()).into_response()
        }
    }
}

#[derive(serde::Deserialize)]
struct SearchMemoriesRequest {
    query: String,
    tag: Option<String>,
}

async fn search_memories(
    State(state): State<Arc<ServerState>>,
    Json(req): Json<SearchMemoriesRequest>,
) -> impl IntoResponse {
    let result = if let Some(tag) = req.tag {
        state.memory_store.search_by_tag(&tag)
    } else {
        state.memory_store.search(&req.query)
    };
    
    match result {
        Ok(memories) => (StatusCode::OK, Json(memories)).into_response(),
        Err(err) => {
            error!("Error searching memories: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

// Wrapper functions to ensure correct type signatures for the router
#[axum::debug_handler]
async fn list_memories_handler(
    state: State<Arc<ServerState>>,
) -> impl IntoResponse {
    list_memories(state).await
}

#[axum::debug_handler]
async fn get_memory_handler(
    state: State<Arc<ServerState>>,
    path: Path<String>,
) -> impl IntoResponse {
    get_memory(state, path).await
}

#[axum::debug_handler]
async fn create_memory_handler(
    state: State<Arc<ServerState>>,
    json: Json<CreateMemoryRequest>,
) -> impl IntoResponse {
    create_memory(state, json).await
}

#[axum::debug_handler]
async fn delete_memory_handler(
    state: State<Arc<ServerState>>,
    path: Path<String>,
) -> impl IntoResponse {
    delete_memory(state, path).await
}

#[axum::debug_handler]
async fn search_memories_handler(
    state: State<Arc<ServerState>>,
    json: Json<SearchMemoriesRequest>,
) -> impl IntoResponse {
    search_memories(state, json).await
}
