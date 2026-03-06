use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the current working context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// The main note describing what the user is working on
    pub note: String,

    /// When this context was created/updated
    pub timestamp: DateTime<Utc>,

    /// Current git repository (if in a git repo)
    pub repo: Option<String>,

    /// Current git branch (if in a git repo)
    pub branch: Option<String>,
}

impl Context {
    /// Create a new context with the given note
    pub fn new(note: String) -> Self {
        Self {
            note,
            timestamp: Utc::now(),
            repo: None,
            branch: None,
        }
    }

    /// Create a new context with git information
    pub fn new_with_git(note: String, repo: String, branch: String) -> Self {
        Self {
            note,
            timestamp: Utc::now(),
            repo: Some(repo),
            branch: Some(branch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_new() {
        let context = Context::new("test note".to_string());
        assert_eq!(context.note, "test note");
        assert!(context.repo.is_none());
        assert!(context.branch.is_none());
    }

    #[test]
    fn test_context_new_with_git() {
        let context = Context::new_with_git(
            "test note".to_string(),
            "my-repo".to_string(),
            "feature-branch".to_string(),
        );
        assert_eq!(context.note, "test note");
        assert_eq!(context.repo, Some("my-repo".to_string()));
        assert_eq!(context.branch, Some("feature-branch".to_string()));
    }

    #[test]
    fn test_context_serialization() {
        let context =
            Context::new_with_git("test".to_string(), "repo".to_string(), "branch".to_string());

        // Serialize
        let json = serde_json::to_string(&context).unwrap();

        // Deserialize
        let deserialized: Context = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.note, "test");
        assert_eq!(deserialized.repo, Some("repo".to_string()));
        assert_eq!(deserialized.branch, Some("branch".to_string()));
    }
}
