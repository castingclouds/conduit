use std::net::SocketAddr;
use conduit_backend::ConduitBackend;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_api_server(docs_path: Option<String>, port: u16) -> Result<String, String> {
    tracing::info!("[TAURI] Starting API server with port: {}", port);
    
    // Initialize the backend with the provided docs_path
    let backend = match ConduitBackend::new(docs_path) {
        Ok(backend) => backend,
        Err(e) => {
            let err_msg = format!("Failed to initialize backend: {}", e);
            tracing::error!("[TAURI] {}", err_msg);
            return Err(err_msg);
        }
    };
    
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("[TAURI] Created backend and address: {}", addr);
    
    match backend.start_server(addr).await {
        Ok(_) => Ok(format!("API server started on http://{}", addr)),
        Err(e) => Err(format!("Failed to start API server: {}", e)),
    }
}

#[tauri::command]
async fn create_memory(title: String, content: String, tags: Vec<String>, docs_path: Option<String>) -> Result<String, String> {
    // Initialize the backend with the provided docs_path
    let backend = ConduitBackend::new(docs_path)?;
    
    // Create the memory using the backend
    backend.create_memory(title, content, tags)
}

#[tauri::command]
async fn get_memory(id: String, docs_path: Option<String>) -> Result<conduit_backend::memory::Memory, String> {
    // Initialize the backend with the provided docs_path
    let backend = ConduitBackend::new(docs_path)?;
    
    // Get the memory using the backend
    backend.get_memory(&id)
}

#[tauri::command]
async fn list_memories(docs_path: Option<String>) -> Result<Vec<conduit_backend::memory::Memory>, String> {
    // Initialize the backend with the provided docs_path
    let backend = ConduitBackend::new(docs_path)?;
    
    // List memories using the backend
    backend.list_memories()
}

#[tauri::command]
async fn search_memories(query: String, docs_path: Option<String>) -> Result<Vec<conduit_backend::memory::Memory>, String> {
    // Initialize the backend with the provided docs_path
    let backend = ConduitBackend::new(docs_path)?;
    
    // Search memories using the backend
    backend.search_memories(&query)
}

#[tauri::command]
async fn delete_memory(id: String, docs_path: Option<String>) -> Result<(), String> {
    // Initialize the backend with the provided docs_path
    let backend = ConduitBackend::new(docs_path)?;
    
    // Delete the memory using the backend
    backend.delete_memory(&id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();
    
    // Start the API server on a separate thread
    let port = 3000;
    
    // Initialize the backend with default memory path
    let backend = match ConduitBackend::new(None) {
        Ok(backend) => backend,
        Err(e) => {
            eprintln!("Failed to initialize backend: {}", e);
            panic!("Failed to initialize backend: {}", e);
        }
    };
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port)); // Use 0.0.0.0 to allow external connections
    tracing::info!("[MAIN] Created backend and address: {}", addr);
    
    // Create a runtime for the API server
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");
    
    // Start the API server in the background
    let _server_handle = rt.spawn(async move {
        tracing::info!("[MAIN] Starting API server on {}", addr);
        match backend.start_server(addr).await {
            Ok(_) => {
                tracing::info!("[MAIN] API server started on http://{}", addr);
                println!("API server started on http://{}", addr);
            },
            Err(e) => {
                tracing::error!("[MAIN] Failed to start API server: {}", e);
                eprintln!("Failed to start API server: {}", e);
            },
        }
    });
    
    // Give the server a moment to start up
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            start_api_server,
            create_memory,
            get_memory,
            list_memories,
            search_memories,
            delete_memory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
