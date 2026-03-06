use anyhow::Result;
use git2::Repository;
use std::path::Path;

/// Git information for the current directory
pub struct GitInfo {
    pub repo_name: String,
    pub branch: String,
}

/// Detect git information for the current directory
pub fn detect_git_info(path: &Path) -> Result<Option<GitInfo>> {
    // Try to find a git repository
    let repo = match Repository::discover(path) {
        Ok(repo) => repo,
        Err(_) => return Ok(None), // Not in a git repo
    };

    // Get the repository name (directory name)
    let repo_path = repo
        .path()
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Could not determine repository path"))?;
    let repo_name = repo_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Could not determine repository name"))?
        .to_string_lossy()
        .to_string();

    // Get the current branch
    let branch = match repo.head() {
        Ok(head) => head
            .shorthand()
            .ok_or_else(|| anyhow::anyhow!("Could not determine current branch"))?
            .to_string(),
        Err(_) => {
            // Unborn branch (no commits yet), default to "master"
            // This happens when git init is done but no commits exist
            "master".to_string()
        }
    };

    Ok(Some(GitInfo { repo_name, branch }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_git_info_in_repo() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Initialize a git repo
        Repository::init(path).unwrap();

        // Create a test file and commit
        fs::write(path.join("test.txt"), "test").unwrap();
        let repo = Repository::open(path).unwrap();
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test.txt")).unwrap();
        index.write().unwrap();

        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap();

        let info = detect_git_info(path).unwrap();
        assert!(info.is_some());
        let info = info.unwrap();
        // Branch name depends on git config (init.defaultBranch)
        assert!(matches!(info.branch.as_str(), "master" | "main"));
    }

    #[test]
    fn test_detect_git_info_not_in_repo() {
        let temp_dir = TempDir::new().unwrap();
        let info = detect_git_info(temp_dir.path()).unwrap();
        assert!(info.is_none());
    }

    #[test]
    fn test_detect_git_info_unborn_branch() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Initialize a git repo without any commits (unborn branch)
        Repository::init(path).unwrap();

        let info = detect_git_info(path).unwrap();
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.branch, "master"); // Should default to master
    }
}
