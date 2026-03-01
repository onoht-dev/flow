use crate::context::Context;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// A completed context entry in history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// The context that was completed
    pub context: Context,
    /// When this context was marked as done
    pub completed_at: DateTime<Utc>,
    /// Duration in minutes (approximate)
    pub duration_minutes: i64,
}

/// Handles persistence of context data
pub struct Storage {
    /// Base directory for storing data
    base_dir: PathBuf,
}

impl Storage {
    /// Create a new storage instance
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Get the default storage location (~/.flow)
    pub fn default_location() -> Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        Ok(Self::new(home.join(".flow")))
    }

    /// Get the path to the current context file
    fn context_file(&self) -> PathBuf {
        self.base_dir.join("context.json")
    }

    /// Get the path to the history file
    fn history_file(&self) -> PathBuf {
        self.base_dir.join("history.json")
    }

    /// Ensure the storage directory exists
    fn ensure_dir(&self) -> Result<()> {
        if !self.base_dir.exists() {
            fs::create_dir_all(&self.base_dir)?;
        }
        Ok(())
    }

    /// Save the current context
    pub fn save_context(&self, context: &Context) -> Result<()> {
        self.ensure_dir()?;
        let path = self.context_file();
        let json = serde_json::to_string_pretty(context)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load the current context
    pub fn load_context(&self) -> Result<Option<Context>> {
        let path = self.context_file();
        if !path.exists() {
            return Ok(None);
        }

        let json = fs::read_to_string(path)?;
        let context: Context = serde_json::from_str(&json)?;
        Ok(Some(context))
    }

    /// Clear the current context
    pub fn clear_context(&self) -> Result<()> {
        let path = self.context_file();
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Append a context to history
    pub fn append_to_history(&self, context: &Context) -> Result<()> {
        self.ensure_dir()?;

        let duration = Utc::now().signed_duration_since(context.timestamp);
        let duration_minutes = duration.num_minutes().max(0);

        let entry = HistoryEntry {
            context: context.clone(),
            completed_at: Utc::now(),
            duration_minutes,
        };

        // Load existing history
        let mut history = self.load_history()?;

        // Add new entry at the beginning (most recent first)
        history.insert(0, entry);

        // Keep only last 100 entries
        if history.len() > 100 {
            history.truncate(100);
        }

        // Save history
        let path = self.history_file();
        let json = serde_json::to_string_pretty(&history)?;
        fs::write(path, json)?;

        Ok(())
    }

    /// Load history entries
    pub fn load_history(&self) -> Result<Vec<HistoryEntry>> {
        let path = self.history_file();
        if !path.exists() {
            return Ok(Vec::new());
        }

        let json = fs::read_to_string(path)?;
        let history: Vec<HistoryEntry> = serde_json::from_str(&json)?;
        Ok(history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_context() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let context = Context::new("testing auth bug".to_string());
        storage.save_context(&context).unwrap();

        let loaded = storage.load_context().unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.note, "testing auth bug");
    }

    #[test]
    fn test_load_nonexistent_context() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let loaded = storage.load_context().unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_clear_context() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let context = Context::new("testing".to_string());
        storage.save_context(&context).unwrap();

        storage.clear_context().unwrap();
        let loaded = storage.load_context().unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_save_creates_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let non_existent_dir = temp_dir.path().join("nested").join("dirs");
        let storage = Storage::new(non_existent_dir.clone());

        // Directory doesn't exist yet
        assert!(!non_existent_dir.exists());

        let context = Context::new("test".to_string());
        storage.save_context(&context).unwrap();

        // Directory should be created
        assert!(non_existent_dir.exists());
    }

    #[test]
    fn test_context_with_special_characters() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let note = "Testing with special chars: émojis 🎉, quotes \"'\", newlines\n, and tabs\t";
        let context = Context::new(note.to_string());
        storage.save_context(&context).unwrap();

        let loaded = storage.load_context().unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.note, note);
    }
}
