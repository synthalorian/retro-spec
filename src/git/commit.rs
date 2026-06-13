use anyhow::Result;

/// Extracted metadata for a single commit
#[derive(Clone, Debug)]
pub struct CommitInfo {
    pub id: String,
    pub author: String,
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
    pub commit_type: String, // feat, fix, docs, chore, refactor, test, style, other
}

/// Extract structured information from a git2::Commit
pub fn extract_commit_info(
    repo: &git2::Repository,
    commit: &git2::Commit,
) -> Result<CommitInfo> {
    let id = commit.id().to_string();
    let author = commit.author().name().unwrap_or("unknown").to_string();
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

    let commit_type = classify_commit_type(&message);

    Ok(CommitInfo {
        id,
        author,
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
        commit_type,
    })
}

/// Classify a commit by its message prefix into a type category
fn classify_commit_type(message: &str) -> String {
    let first_line = message.lines().next().unwrap_or("").to_lowercase();
    if first_line.starts_with("feat") || first_line.starts_with("feature") {
        "feat".to_string()
    } else if first_line.starts_with("fix") || first_line.starts_with("bug")
        || first_line.starts_with("hotfix") || first_line.starts_with("patch")
    {
        "fix".to_string()
    } else if first_line.starts_with("docs") || first_line.starts_with("doc")
        || first_line.starts_with("readme")
    {
        "docs".to_string()
    } else if first_line.starts_with("refactor") || first_line.starts_with("refact")
        || first_line.starts_with("clean") || first_line.starts_with("rewrite")
    {
        "refactor".to_string()
    } else if first_line.starts_with("chore") || first_line.starts_with("build")
        || first_line.starts_with("ci") || first_line.starts_with("dep")
        || first_line.starts_with("config")
    {
        "chore".to_string()
    } else if first_line.starts_with("test") || first_line.starts_with("spec")
    {
        "test".to_string()
    } else if first_line.starts_with("style") || first_line.starts_with("fmt")
        || first_line.starts_with("lint") || first_line.starts_with("format")
    {
        "style".to_string()
    } else if first_line.starts_with("perf") || first_line.starts_with("optimize")
    {
        "perf".to_string()
    } else {
        "other".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_commit_type_feat() {
        assert_eq!(classify_commit_type("feat: add new feature"), "feat");
        assert_eq!(classify_commit_type("feature: add new feature"), "feat");
    }

    #[test]
    fn test_classify_commit_type_fix() {
        assert_eq!(classify_commit_type("fix: bug fix"), "fix");
        assert_eq!(classify_commit_type("bug: fix something"), "fix");
        assert_eq!(classify_commit_type("hotfix: urgent fix"), "fix");
        assert_eq!(classify_commit_type("patch: small fix"), "fix");
    }

    #[test]
    fn test_classify_commit_type_docs() {
        assert_eq!(classify_commit_type("docs: update readme"), "docs");
        assert_eq!(classify_commit_type("doc: add documentation"), "docs");
        assert_eq!(classify_commit_type("readme: update readme"), "docs");
    }

    #[test]
    fn test_classify_commit_type_other() {
        assert_eq!(classify_commit_type("random message"), "other");
        assert_eq!(classify_commit_type(""), "other");
    }
}
