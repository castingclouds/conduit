use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Memory not found: {0}")]
    NotFound(String),
    
    #[error("Invalid memory format: {0}")]
    InvalidFormat(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Memory {
    pub fn new(title: String, content: String, tags: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            tags,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        
        // Add YAML frontmatter
        md.push_str("---\n");
        md.push_str(&format!("id: {}\n", self.id));
        md.push_str(&format!("title: {}\n", self.title));
        md.push_str(&format!("tags: [{}]\n", self.tags.join(", ")));
        md.push_str(&format!("created_at: {}\n", self.created_at.to_rfc3339()));
        md.push_str(&format!("updated_at: {}\n", self.updated_at.to_rfc3339()));
        md.push_str("---\n\n");
        
        // Add content
        md.push_str(&self.content);
        
        md
    }
    
    pub fn from_markdown(markdown: &str) -> Result<Self, MemoryError> {
        let re = regex::Regex::new(r"(?s)---\n(.*?)\n---\n\n(.*)").unwrap();
        
        if let Some(captures) = re.captures(markdown) {
            let frontmatter = captures.get(1).unwrap().as_str();
            let content = captures.get(2).unwrap().as_str();
            
            // Parse frontmatter
            let id_re = regex::Regex::new(r"id: (.*)").unwrap();
            let title_re = regex::Regex::new(r"title: (.*)").unwrap();
            let tags_re = regex::Regex::new(r"tags: \[(.*)\]").unwrap();
            let created_at_re = regex::Regex::new(r"created_at: (.*)").unwrap();
            let updated_at_re = regex::Regex::new(r"updated_at: (.*)").unwrap();
            
            let id = id_re.captures(frontmatter)
                .ok_or_else(|| MemoryError::InvalidFormat("Missing id".to_string()))?
                .get(1).unwrap().as_str().to_string();
                
            let title = title_re.captures(frontmatter)
                .ok_or_else(|| MemoryError::InvalidFormat("Missing title".to_string()))?
                .get(1).unwrap().as_str().to_string();
                
            let tags_str = tags_re.captures(frontmatter)
                .ok_or_else(|| MemoryError::InvalidFormat("Missing tags".to_string()))?
                .get(1).unwrap().as_str();
                
            let tags: Vec<String> = tags_str.split(',')
                .map(|s| s.trim().to_string())
                .collect();
                
            let created_at_str = created_at_re.captures(frontmatter)
                .ok_or_else(|| MemoryError::InvalidFormat("Missing created_at".to_string()))?
                .get(1).unwrap().as_str();
                
            // Try parsing with different date formats
            let created_at = match DateTime::parse_from_rfc3339(created_at_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => {
                    // Try alternative formats
                    if let Ok(dt) = chrono::DateTime::parse_from_str(created_at_str, "%Y-%m-%d %H:%M:%S%.f %z") {
                        dt.with_timezone(&Utc)
                    } else if let Ok(dt) = chrono::DateTime::parse_from_str(created_at_str, "%Y-%m-%d %H:%M:%S %z") {
                        dt.with_timezone(&Utc)
                    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(created_at_str, "%Y-%m-%d %H:%M:%S") {
                        // Assume UTC if no timezone is specified
                        chrono::DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)
                    } else {
                        return Err(MemoryError::InvalidFormat(format!("Invalid created_at format: {}", created_at_str)));
                    }
                }
            };
                
            let updated_at_str = updated_at_re.captures(frontmatter)
                .ok_or_else(|| MemoryError::InvalidFormat("Missing updated_at".to_string()))?
                .get(1).unwrap().as_str();
                
            // Try parsing with different date formats
            let updated_at = match DateTime::parse_from_rfc3339(updated_at_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => {
                    // Try alternative formats
                    if let Ok(dt) = chrono::DateTime::parse_from_str(updated_at_str, "%Y-%m-%d %H:%M:%S%.f %z") {
                        dt.with_timezone(&Utc)
                    } else if let Ok(dt) = chrono::DateTime::parse_from_str(updated_at_str, "%Y-%m-%d %H:%M:%S %z") {
                        dt.with_timezone(&Utc)
                    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(updated_at_str, "%Y-%m-%d %H:%M:%S") {
                        // Assume UTC if no timezone is specified
                        chrono::DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)
                    } else {
                        return Err(MemoryError::InvalidFormat(format!("Invalid updated_at format: {}", updated_at_str)));
                    }
                }
            };
            
            Ok(Self {
                id,
                title,
                content: content.to_string(),
                tags,
                created_at,
                updated_at,
            })
        } else {
            Err(MemoryError::InvalidFormat("Invalid markdown format".to_string()))
        }
    }
}

pub struct MemoryStore {
    pub base_path: PathBuf,
}

impl MemoryStore {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        let path = base_path.as_ref().to_path_buf();
        
        // Create directory if it doesn't exist
        if !path.exists() {
            fs::create_dir_all(&path).expect("Failed to create memory directory");
        }
        
        let store = Self { base_path: path };
        
        // Try to fix any existing memory files with invalid date formats
        let _ = store.fix_invalid_memory_files();
        
        store
    }
    
    // Fix any existing memory files with invalid date formats
    fn fix_invalid_memory_files(&self) -> Result<(), MemoryError> {
        if !self.base_path.exists() {
            return Ok(());
        }
        
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                let mut file = File::open(&path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                
                // Try to parse the memory file
                match Memory::from_markdown(&content) {
                    Ok(_) => {}, // File is valid, no need to fix
                    Err(e) => {
                        // Only try to fix invalid date format errors
                        if let MemoryError::InvalidFormat(msg) = &e {
                            if msg.contains("Invalid created_at format") || msg.contains("Invalid updated_at format") {
                                // Try to extract the memory data and rewrite the file with valid date formats
                                if let Some(fixed_memory) = self.try_fix_memory_file(&content) {
                                    // Save the fixed memory back to the file
                                    let markdown = fixed_memory.to_markdown();
                                    let mut file = File::create(&path)?;
                                    file.write_all(markdown.as_bytes())?;
                                    println!("Fixed memory file: {:?}", path);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    // Try to fix a memory file with invalid date formats
    fn try_fix_memory_file(&self, content: &str) -> Option<Memory> {
        let re = regex::Regex::new(r"(?s)---\n(.*)\n---\n\n(.*)").unwrap();
        
        if let Some(captures) = re.captures(content) {
            let frontmatter = captures.get(1)?.as_str();
            let content_str = captures.get(2)?.as_str();
            
            // Parse frontmatter
            let id_re = regex::Regex::new(r"id: (.*)").unwrap();
            let title_re = regex::Regex::new(r"title: (.*)").unwrap();
            let tags_re = regex::Regex::new(r"tags: \[(.*?)\]").unwrap();
            
            let id = id_re.captures(frontmatter)?.get(1)?.as_str().to_string();
            let title = title_re.captures(frontmatter)?.get(1)?.as_str().to_string();
            
            let tags_str = tags_re.captures(frontmatter)?.get(1)?.as_str();
            let tags: Vec<String> = tags_str.split(',').map(|s| s.trim().to_string()).collect();
            
            // Create a new memory with current timestamps
            let now = Utc::now();
            let memory = Memory {
                id,
                title,
                content: content_str.to_string(),
                tags,
                created_at: now,
                updated_at: now,
            };
            
            return Some(memory);
        }
        
        None
    }
    
    fn get_memory_path(&self, id: &str) -> PathBuf {
        self.base_path.join(format!("{}.md", id))
    }
    
    pub fn save(&self, memory: &Memory) -> Result<(), MemoryError> {
        let path = self.get_memory_path(&memory.id);
        let markdown = memory.to_markdown();
        
        let mut file = File::create(path)?;
        file.write_all(markdown.as_bytes())?;
        
        Ok(())
    }
    
    pub fn get(&self, id: &str) -> Result<Memory, MemoryError> {
        let path = self.get_memory_path(id);
        
        if !path.exists() {
            return Err(MemoryError::NotFound(id.to_string()));
        }
        
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        Memory::from_markdown(&content)
    }
    
    pub fn delete(&self, id: &str) -> Result<(), MemoryError> {
        let path = self.get_memory_path(id);
        
        if !path.exists() {
            return Err(MemoryError::NotFound(id.to_string()));
        }
        
        fs::remove_file(path)?;
        
        Ok(())
    }
    
    pub fn list(&self) -> Result<Vec<Memory>, MemoryError> {
        println!("[DEBUG] Listing memories from path: {:?}", self.base_path);
        let mut memories = Vec::new();
        
        if !self.base_path.exists() {
            println!("[DEBUG] Memory directory does not exist, creating it");
            fs::create_dir_all(&self.base_path)?;
            return Ok(memories);
        }
        
        match fs::read_dir(&self.base_path) {
            Ok(entries) => {
                for entry_result in entries {
                    match entry_result {
                        Ok(entry) => {
                            let path = entry.path();
                            println!("[DEBUG] Processing file: {:?}", path);
                            
                            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                                match File::open(&path) {
                                    Ok(mut file) => {
                                        let mut content = String::new();
                                        match file.read_to_string(&mut content) {
                                            Ok(_) => {
                                                match Memory::from_markdown(&content) {
                                                    Ok(memory) => {
                                                        println!("[DEBUG] Successfully parsed memory: {}", memory.id);
                                                        memories.push(memory);
                                                    },
                                                    Err(e) => {
                                                        println!("[DEBUG] Error parsing memory file {:?}: {:?}", path, e);
                                                        // Try to recover the memory if possible
                                                        if let Some(fixed_memory) = self.try_fix_memory_file(&content) {
                                                            println!("[DEBUG] Recovered memory: {}", fixed_memory.id);
                                                            memories.push(fixed_memory);
                                                        }
                                                    }
                                                }
                                            },
                                            Err(e) => println!("[DEBUG] Error reading file {:?}: {:?}", path, e)
                                        }
                                    },
                                    Err(e) => println!("[DEBUG] Error opening file {:?}: {:?}", path, e)
                                }
                            }
                        },
                        Err(e) => println!("[DEBUG] Error accessing directory entry: {:?}", e)
                    }
                }
            },
            Err(e) => {
                println!("[DEBUG] Error reading directory {:?}: {:?}", self.base_path, e);
                return Err(MemoryError::Io(e));
            }
        }
        
        println!("[DEBUG] Found {} memories", memories.len());
        Ok(memories)
    }
    
    pub fn search(&self, query: &str) -> Result<Vec<Memory>, MemoryError> {
        let memories = self.list()?;
        
        let query = query.to_lowercase();
        let filtered = memories.into_iter()
            .filter(|memory| {
                memory.title.to_lowercase().contains(&query) ||
                memory.content.to_lowercase().contains(&query) ||
                memory.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .collect();
            
        Ok(filtered)
    }
    
    pub fn search_by_tag(&self, tag: &str) -> Result<Vec<Memory>, MemoryError> {
        let memories = self.list()?;
        
        let tag = tag.to_lowercase();
        let filtered = memories.into_iter()
            .filter(|memory| {
                memory.tags.iter().any(|t| t.to_lowercase() == tag)
            })
            .collect();
            
        Ok(filtered)
    }
}
