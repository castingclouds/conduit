use std::net::SocketAddr;
use std::sync::Arc;
use std::path::Path;

pub mod api;
pub mod memory;

/// The main entry point for the Conduit backend.
/// 
/// This struct provides a clean API for interacting with the memory store
/// and starting the API server.
pub struct ConduitBackend {
    memory_store: Arc<memory::MemoryStore>,
}

impl ConduitBackend {
    /// Create a new ConduitBackend instance
    ///
    /// # Arguments
    ///
    /// * `memory_path` - Optional path to the memory storage directory. If not provided,
    ///   the default location (~/.conduit/memories) will be used.
    ///
    /// # Returns
    ///
    /// A Result containing the ConduitBackend instance or an error message.
    ///
    /// # Example
    ///
    /// ```
    /// use conduit_backend::ConduitBackend;
    ///
    /// // Use default memory path (~/.conduit/memories)
    /// let backend = ConduitBackend::new(None).unwrap();
    ///
    /// // Or specify a custom path
    /// let backend = ConduitBackend::new(Some("/path/to/memories".to_string())).unwrap();
    /// ```
    pub fn new(memory_path: Option<String>) -> Result<Self, String> {
        // Set up the memory directory in the user's home directory if not provided
        let memory_path = if let Some(path) = memory_path {
            tracing::info!("Using provided memory_path: {}", path);
            path
        } else {
            let home_dir = dirs::home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
            let memory_dir = home_dir.join(".conduit").join("memories");
            let path = memory_dir.to_string_lossy().to_string();
            tracing::info!("Using default memory path: {}", path);
            path
        };
        
        // Ensure the memory directory exists
        let path = Path::new(&memory_path);
        if !path.exists() {
            tracing::info!("Memory directory does not exist, creating it: {}", memory_path);
            std::fs::create_dir_all(path).map_err(|e| format!("Failed to create memory directory: {}", e))?;
        }
        
        let memory_store = Arc::new(memory::MemoryStore::new(memory_path));
        Ok(Self { memory_store })
    }
    
    /// Start the API server
    ///
    /// # Arguments
    ///
    /// * `addr` - The socket address to bind the server to
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error message.
    ///
    /// # Example
    ///
    /// ```
    /// use conduit_backend::ConduitBackend;
    /// use std::net::SocketAddr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), String> {
    ///     let backend = ConduitBackend::new(None)?;
    ///     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    ///     backend.start_server(addr).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn start_server(&self, addr: SocketAddr) -> Result<(), String> {
        // The start_server function returns a shutdown sender, but we don't need to expose that
        // in our public API. We'll just return success if the server started successfully.
        match api::server::start_server(self.memory_store.clone(), addr).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    
    /// Create a new memory
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the memory
    /// * `content` - The content of the memory
    /// * `tags` - A vector of tags for the memory
    ///
    /// # Returns
    ///
    /// A Result containing the ID of the created memory or an error message.
    pub fn create_memory(&self, title: String, content: String, tags: Vec<String>) -> Result<String, String> {
        let memory = memory::Memory::new(title, content, tags);
        self.memory_store.save(&memory).map_err(|e| e.to_string())?;
        Ok(memory.id)
    }
    
    /// Get a memory by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the memory to retrieve
    ///
    /// # Returns
    ///
    /// A Result containing the Memory or an error message.
    pub fn get_memory(&self, id: &str) -> Result<memory::Memory, String> {
        self.memory_store.get(id).map_err(|e| e.to_string())
    }
    
    /// List all memories
    ///
    /// # Returns
    ///
    /// A Result containing a vector of all memories or an error message.
    pub fn list_memories(&self) -> Result<Vec<memory::Memory>, String> {
        self.memory_store.list().map_err(|e| e.to_string())
    }
    
    /// Search memories
    ///
    /// # Arguments
    ///
    /// * `query` - The search query
    ///
    /// # Returns
    ///
    /// A Result containing a vector of matching memories or an error message.
    pub fn search_memories(&self, query: &str) -> Result<Vec<memory::Memory>, String> {
        self.memory_store.search(query).map_err(|e| e.to_string())
    }
    
    /// Delete a memory by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the memory to delete
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error message.
    pub fn delete_memory(&self, id: &str) -> Result<(), String> {
        self.memory_store.delete(id).map_err(|e| e.to_string())
    }
    
    /// Get the memory store
    ///
    /// This method provides direct access to the memory store for advanced usage.
    ///
    /// # Returns
    ///
    /// A clone of the Arc-wrapped MemoryStore.
    pub fn memory_store(&self) -> Arc<memory::MemoryStore> {
        self.memory_store.clone()
    }
}
