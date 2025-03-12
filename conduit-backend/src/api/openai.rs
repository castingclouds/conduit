 use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tracing::{info, error};

use crate::memory::Memory;
use super::state::ServerState;

pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/models", get(list_models_handler))
        .route("/chat/completions", post(chat_completions_handler))
        .route("/embeddings", post(create_embeddings_handler))
        .route("/memories", get(list_memories_handler).post(create_memory_handler))
        .route("/memories/:id", get(get_memory_handler).delete(delete_memory_handler))
}

// Wrapper functions to ensure correct type signatures for the router
#[axum::debug_handler]
async fn list_models_handler(
    state: State<Arc<ServerState>>,
) -> impl IntoResponse {
    list_models(state).await
}

#[axum::debug_handler]
async fn chat_completions_handler(
    state: State<Arc<ServerState>>,
    json: Json<ChatCompletionRequest>,
) -> impl IntoResponse {
    chat_completions(state, json).await
}

#[axum::debug_handler]
async fn create_embeddings_handler(
    state: State<Arc<ServerState>>,
    json: Json<EmbeddingRequest>,
) -> impl IntoResponse {
    create_embeddings(state, json).await
}

#[axum::debug_handler]
async fn list_memories_handler(
    state: State<Arc<ServerState>>,
) -> impl IntoResponse {
    list_memories(state).await
}

#[axum::debug_handler]
async fn create_memory_handler(
    state: State<Arc<ServerState>>,
    json: Json<MemoryRequest>,
) -> impl IntoResponse {
    create_memory(state, json).await
}

#[axum::debug_handler]
async fn get_memory_handler(
    state: State<Arc<ServerState>>,
    path: axum::extract::Path<String>,
) -> impl IntoResponse {
    get_memory(state, path).await
}

#[axum::debug_handler]
async fn delete_memory_handler(
    state: State<Arc<ServerState>>,
    path: axum::extract::Path<String>,
) -> impl IntoResponse {
    delete_memory(state, path).await
}

// OpenAI API compatible types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelList {
    pub object: String,
    pub data: Vec<Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: usize,
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: ChatCompletionUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub index: usize,
    pub object: String,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: EmbeddingUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingUsage {
    pub prompt_tokens: i32,
    pub total_tokens: i32,
}

// Memory types for OpenAI API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRequest {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// API handlers
async fn list_models(
    State(_state): State<Arc<ServerState>>,
) -> impl IntoResponse {
    let models = ModelList {
        object: "list".to_string(),
        data: vec![
            Model {
                id: "gpt-3.5-turbo".to_string(),
                object: "model".to_string(),
                created: Utc::now().timestamp(),
                owned_by: "conduit".to_string(),
            },
            Model {
                id: "text-embedding-ada-002".to_string(),
                object: "model".to_string(),
                created: Utc::now().timestamp(),
                owned_by: "conduit".to_string(),
            },
        ],
    };
    
    (StatusCode::OK, Json(models)).into_response()
}

async fn chat_completions(
    State(state): State<Arc<ServerState>>,
    Json(req): Json<ChatCompletionRequest>,
) -> impl IntoResponse {
    info!("Chat completion request for model: {}", req.model);
    
    // Process the chat request
    // In a real implementation, this would call an actual LLM
    // For now, we'll just echo back the last message with some context
    
    let memories = match state.memory_store.list() {
        Ok(mems) => mems,
        Err(err) => {
            error!("Error retrieving memories: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": {
                        "message": format!("Failed to retrieve memories: {}", err),
                        "type": "internal_error"
                    }
                }))
            ).into_response();
        }
    };
    
    // Get the last user message
    let last_message = req.messages.last().cloned().unwrap_or(ChatMessage {
        role: "user".to_string(),
        content: "Hello".to_string(),
    });
    
    // Create a simple response that mentions the available memories
    let memory_titles: Vec<String> = memories.iter()
        .map(|m| format!("- {}", m.title))
        .collect();
    
    let memory_count = memories.len();
    let response_content = format!(
        "I received your message: '{}'\n\nI have access to {} memories:\n{}\n\nHow can I help you with these memories?",
        last_message.content,
        memory_count,
        memory_titles.join("\n")
    );
    
    // Create the completion response
    let completion = ChatCompletionResponse {
        id: format!("chatcmpl-{}", Uuid::new_v4()),
        object: "chat.completion".to_string(),
        created: Utc::now().timestamp(),
        model: req.model,
        choices: vec![
            ChatCompletionChoice {
                index: 0,
                message: ChatMessage {
                    role: "assistant".to_string(),
                    content: response_content,
                },
                finish_reason: "stop".to_string(),
            }
        ],
        usage: ChatCompletionUsage {
            prompt_tokens: 100, // Placeholder values
            completion_tokens: 100,
            total_tokens: 200,
        },
    };
    
    (StatusCode::OK, Json(completion)).into_response()
}

async fn create_embeddings(
    State(_state): State<Arc<ServerState>>,
    Json(req): Json<EmbeddingRequest>,
) -> impl IntoResponse {
    info!("Embedding request for model: {}", req.model);
    
    // In a real implementation, this would call an actual embedding model
    // For now, we'll just return random embeddings
    
    let mut embeddings = Vec::new();
    
    for (i, text) in req.input.iter().enumerate() {
        // Create a deterministic but simple embedding based on the text length
        // This is just a placeholder - real embeddings would come from a model
        let mut embedding = Vec::new();
        let seed = text.len() as f32;
        
        for j in 0..10 {
            // Generate a simple deterministic value based on text and position
            let val = ((j as f32 * 0.1) + seed * 0.01).sin();
            embedding.push(val);
        }
        
        embeddings.push(EmbeddingData {
            index: i,
            object: "embedding".to_string(),
            embedding,
        });
    }
    
    let response = EmbeddingResponse {
        object: "list".to_string(),
        data: embeddings,
        model: req.model,
        usage: EmbeddingUsage {
            prompt_tokens: req.input.iter().map(|s| s.len() as i32 / 4).sum(),
            total_tokens: req.input.iter().map(|s| s.len() as i32 / 4).sum(),
        },
    };
    
    (StatusCode::OK, Json(response)).into_response()
}

// Memory API handlers for OpenAI API path
async fn list_memories(
    State(state): State<Arc<ServerState>>,
) -> impl IntoResponse {
    info!("[API] Handling list_memories request");
    
    // Ensure the memory directory exists
    let base_path = state.memory_store.base_path.clone();
    info!("[API] Memory base path: {:?}", base_path);
    
    if !base_path.exists() {
        info!("[API] Memory directory does not exist, creating it");
        if let Err(e) = std::fs::create_dir_all(&base_path) {
            error!("[API] Failed to create memory directory: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": {
                        "message": format!("Failed to create memory directory: {}", e),
                        "type": "internal_error"
                    }
                }))
            ).into_response();
        }
        info!("[API] Created memory directory");
    }
    
    info!("[API] Calling memory_store.list()");
    match state.memory_store.list() {
        Ok(memories) => {
            let memory_responses: Vec<MemoryResponse> = memories.into_iter()
                .map(|m| MemoryResponse {
                    id: m.id,
                    title: m.title,
                    content: m.content,
                    tags: m.tags,
                    created_at: m.created_at,
                    updated_at: m.updated_at,
                })
                .collect();
                
            (StatusCode::OK, Json(memory_responses)).into_response()
        },
        Err(err) => {
            error!("Error listing memories: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": {
                        "message": format!("Failed to list memories: {}", err),
                        "type": "internal_error"
                    }
                }))
            ).into_response()
        }
    }
}

async fn get_memory(
    State(state): State<Arc<ServerState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    info!("[API] Handling get_memory request for id: {}", id);
    
    // Ensure the memory directory exists
    let base_path = state.memory_store.base_path.clone();
    info!("[API] Memory base path: {:?}", base_path);
    
    if !base_path.exists() {
        info!("[API] Memory directory does not exist, creating it");
        if let Err(e) = std::fs::create_dir_all(&base_path) {
            error!("[API] Failed to create memory directory: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": {
                        "message": format!("Failed to create memory directory: {}", e),
                        "type": "internal_error"
                    }
                }))
            ).into_response();
        }
        info!("[API] Created memory directory");
    }
    
    info!("[API] Calling memory_store.get() for id: {}", id);
    match state.memory_store.get(&id) {
        Ok(memory) => {
            let memory_response = MemoryResponse {
                id: memory.id,
                title: memory.title,
                content: memory.content,
                tags: memory.tags,
                created_at: memory.created_at,
                updated_at: memory.updated_at,
            };
            
            (StatusCode::OK, Json(memory_response)).into_response()
        },
        Err(err) => {
            error!("Error getting memory {}: {:?}", id, err);
            let status = match err {
                crate::memory::MemoryError::NotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            
            (
                status,
                Json(serde_json::json!({
                    "error": {
                        "message": format!("Failed to get memory: {}", err),
                        "type": "not_found"
                    }
                }))
            ).into_response()
        }
    }
}

async fn create_memory(
    State(state): State<Arc<ServerState>>,
    Json(req): Json<MemoryRequest>,
) -> impl IntoResponse {
    info!("[API] Handling create_memory request with title: {}", req.title);
    
    // Ensure the memory directory exists
    let base_path = state.memory_store.base_path.clone();
    info!("[API] Memory base path: {:?}", base_path);
    
    if !base_path.exists() {
        info!("[API] Memory directory does not exist, creating it");
        if let Err(e) = std::fs::create_dir_all(&base_path) {
            error!("[API] Failed to create memory directory: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": {
                        "message": format!("Failed to create memory directory: {}", e),
                        "type": "internal_error"
                    }
                }))
            ).into_response();
        }
        info!("[API] Created memory directory");
    }
    
    info!("[API] Creating new memory with title: {}", req.title);
    let memory = Memory::new(req.title, req.content, req.tags);
    info!("[API] Generated memory ID: {}", memory.id);
    
    match state.memory_store.save(&memory) {
        Ok(_) => {
            let memory_response = MemoryResponse {
                id: memory.id,
                title: memory.title,
                content: memory.content,
                tags: memory.tags,
                created_at: memory.created_at,
                updated_at: memory.updated_at,
            };
            
            (StatusCode::CREATED, Json(memory_response)).into_response()
        },
        Err(err) => {
            error!("Error creating memory: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": {
                        "message": format!("Failed to create memory: {}", err),
                        "type": "internal_error"
                    }
                }))
            ).into_response()
        }
    }
}

async fn delete_memory(
    State(state): State<Arc<ServerState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    info!("[API] Handling delete_memory request for id: {}", id);
    
    // Ensure the memory directory exists
    let base_path = state.memory_store.base_path.clone();
    info!("[API] Memory base path: {:?}", base_path);
    
    if !base_path.exists() {
        info!("[API] Memory directory does not exist, creating it");
        if let Err(e) = std::fs::create_dir_all(&base_path) {
            error!("[API] Failed to create memory directory: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": {
                        "message": format!("Failed to create memory directory: {}", e),
                        "type": "internal_error"
                    }
                }))
            ).into_response();
        }
        info!("[API] Created memory directory");
    }
    
    info!("[API] Calling memory_store.delete() for id: {}", id);
    match state.memory_store.delete(&id) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => {
            error!("Error deleting memory {}: {:?}", id, err);
            let status = match err {
                crate::memory::MemoryError::NotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            
            (
                status,
                Json(serde_json::json!({
                    "error": {
                        "message": format!("Failed to delete memory: {}", err),
                        "type": match err {
                            crate::memory::MemoryError::NotFound(_) => "not_found",
                            _ => "internal_error",
                        }
                    }
                }))
            ).into_response()
        }
    }
}
