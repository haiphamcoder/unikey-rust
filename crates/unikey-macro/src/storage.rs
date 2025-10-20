//! Storage and persistence for macro definitions

use crate::{MacroCollection, MacroDefinition, MacroError, MacroResult};
use serde_json;
use std::fs;
use std::path::Path;

/// Storage backend for macro definitions
pub trait MacroStorage: std::fmt::Debug {
    /// Save macro collection to storage
    fn save(&self, collection: &MacroCollection) -> MacroResult<()>;
    
    /// Load macro collection from storage
    fn load(&self) -> MacroResult<MacroCollection>;
    
    /// Check if storage exists
    fn exists(&self) -> bool;
}

/// File-based storage for macro definitions
#[derive(Debug, Clone)]
pub struct FileMacroStorage {
    /// Path to the storage file
    path: String,
}

impl FileMacroStorage {
    /// Create a new file-based storage
    pub fn new(path: String) -> Self {
        Self { path }
    }
    
    /// Get the storage path
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl MacroStorage for FileMacroStorage {
    fn save(&self, collection: &MacroCollection) -> MacroResult<()> {
        let json = serde_json::to_string_pretty(collection)
            .map_err(|e| MacroError::SerializationError(e.to_string()))?;
        
        fs::write(&self.path, json)
            .map_err(|e| MacroError::StorageError(e.to_string()))?;
        
        Ok(())
    }
    
    fn load(&self) -> MacroResult<MacroCollection> {
        if !self.exists() {
            return Ok(MacroCollection::new());
        }
        
        let content = fs::read_to_string(&self.path)
            .map_err(|e| MacroError::StorageError(e.to_string()))?;
        
        let collection: MacroCollection = serde_json::from_str(&content)
            .map_err(|e| MacroError::SerializationError(e.to_string()))?;
        
        Ok(collection)
    }
    
    fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }
}

/// Memory-based storage for macro definitions
#[derive(Debug, Clone)]
pub struct MemoryMacroStorage {
    /// In-memory collection
    collection: MacroCollection,
}

impl MemoryMacroStorage {
    /// Create a new memory-based storage
    pub fn new() -> Self {
        Self {
            collection: MacroCollection::new(),
        }
    }
    
    /// Create with existing collection
    pub fn with_collection(collection: MacroCollection) -> Self {
        Self { collection }
    }
}

impl MacroStorage for MemoryMacroStorage {
    fn save(&self, collection: &MacroCollection) -> MacroResult<()> {
        // Memory storage doesn't need to do anything
        Ok(())
    }
    
    fn load(&self) -> MacroResult<MacroCollection> {
        Ok(self.collection.clone())
    }
    
    fn exists(&self) -> bool {
        !self.collection.is_empty()
    }
}

/// Macro storage manager
#[derive(Debug)]
pub struct MacroStorageManager {
    /// Primary storage backend
    primary: Box<dyn MacroStorage>,
    /// Backup storage backend
    backup: Option<Box<dyn MacroStorage>>,
}

impl MacroStorageManager {
    /// Create a new storage manager
    pub fn new(primary: Box<dyn MacroStorage>) -> Self {
        Self {
            primary,
            backup: None,
        }
    }
    
    /// Create with backup storage
    pub fn with_backup(primary: Box<dyn MacroStorage>, backup: Box<dyn MacroStorage>) -> Self {
        Self {
            primary,
            backup: Some(backup),
        }
    }
    
    /// Save macro collection
    pub fn save(&self, collection: &MacroCollection) -> MacroResult<()> {
        // Try primary storage first
        match self.primary.save(collection) {
            Ok(()) => Ok(()),
            Err(e) => {
                // Try backup storage if available
                if let Some(backup) = &self.backup {
                    backup.save(collection)
                } else {
                    Err(e)
                }
            }
        }
    }
    
    /// Load macro collection
    pub fn load(&self) -> MacroResult<MacroCollection> {
        // Try primary storage first
        match self.primary.load() {
            Ok(collection) => Ok(collection),
            Err(e) => {
                // Try backup storage if available
                if let Some(backup) = &self.backup {
                    backup.load()
                } else {
                    Err(e)
                }
            }
        }
    }
    
    /// Check if storage exists
    pub fn exists(&self) -> bool {
        self.primary.exists() || self.backup.as_ref().map_or(false, |b| b.exists())
    }
}

/// Macro import/export utilities
pub struct MacroImporter {
    /// Storage manager
    storage_manager: MacroStorageManager,
}

impl MacroImporter {
    /// Create a new importer
    pub fn new(storage_manager: MacroStorageManager) -> Self {
        Self { storage_manager }
    }
    
    /// Import macros from JSON file
    pub fn import_from_file(&self, file_path: &str) -> MacroResult<MacroCollection> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| MacroError::StorageError(e.to_string()))?;
        
        let collection: MacroCollection = serde_json::from_str(&content)
            .map_err(|e| MacroError::SerializationError(e.to_string()))?;
        
        Ok(collection)
    }
    
    /// Export macros to JSON file
    pub fn export_to_file(&self, collection: &MacroCollection, file_path: &str) -> MacroResult<()> {
        let json = serde_json::to_string_pretty(collection)
            .map_err(|e| MacroError::SerializationError(e.to_string()))?;
        
        fs::write(file_path, json)
            .map_err(|e| MacroError::StorageError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Merge collections
    pub fn merge_collections(&self, base: &MacroCollection, other: &MacroCollection) -> MacroCollection {
        let mut merged = base.clone();
        
        for (id, macro_def) in other.get_all_macros() {
            // Skip if already exists (could be made configurable)
            if merged.get_macro(id).is_none() {
                let _ = merged.add_macro(macro_def.clone());
            }
        }
        
        merged
    }
}