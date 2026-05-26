use anyhow::Result;
use chrono::{DateTime, Utc};

/// Extracted metadata for a single commit
#[derive(Clone, Debug)]
pub struct CommitInfo {
    pub id: String,
    pub author: String,
    pub author_email: String,
    pub timestamp: i64,
    pub message: String,
    pub lines_added: u32,
    pub lines_deleted: u32,
    pub files_changed: u32,
    pub branch: String,
    pub parents: Vec<String>,
    pub is_merge: bool,
    pub is_tagged: bool,
    pub tags: Vec<String>,
    pub files: Vec<String>,
}

/// Extract structured information from a git2::Commit
pub fn extract_commit_info(
    repo: &git2::Repository,
    commit: &git2::Commit,
) -> Result<CommitInfo> {
    let id = commit.id().to_string();
    let author = commit.author().name().unwrap_or("unknown").to_string();
    let author_email = commit.author().email().unwrap_or("unknown").to_string();
    let timestamp = commit.time().seconds();
    let message = commit.message().unwrap_or("").to_string();
    let parents: Vec<String> = commit.parents().map(|p| p.id().to_string()).collect();
    let is_merge = parents.len() > 1;

    // Calculate diff stats against first parent
    let (lines_added, lines_deleted, files_changed, files) = if let Ok(tree) = commit.tree() {
        let parent_tree = if let Ok(parent) = commit.parent(0) {
            parent.tree().ok()
        } else {
            None
        };

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;
        let stats = diff.stats()?;

        // Collect changed file paths via the delta iterator
        let mut file_paths = Vec::new();
        diff.deltas().for_each(|delta| {
            if let Some(file) = delta.new_file().path() {
                file_paths.push(file.to_string_lossy().to_string());
            }
        });

        (
            stats.insertions() as u32,
            stats.deletions() as u32,
            stats.files_changed() as u32,
            file_paths,
        )
    } else {
        (0, 0, 0, vec![])
    };

    Ok(CommitInfo {
        id,
        author,
        author_email,
        timestamp,
        message,
        lines_added,
        lines_deleted,
        files_changed,
        branch: String::new(), // populated by dag.rs branch walk
        parents,
        is_merge,
        is_tagged: false,
        tags: Vec::new(),
        files,
    })
}

/// Get the datetime representation of a commit timestamp
pub fn commit_datetime(timestamp: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(timestamp, 0).unwrap_or_default()
}
