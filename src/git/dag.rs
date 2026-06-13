use anyhow::Result;
use std::collections::HashMap;
use git2::Oid;

/// Traverse a git repository's commit DAG — branch-aware
pub struct CommitDag {
    pub commits: Vec<crate::git::commit::CommitInfo>,
    pub branches: Vec<BranchInfo>,
    pub tags: Vec<TagInfo>,
}

pub struct BranchInfo {
    pub name: String,
    pub tip_commit: String,
    pub color: [f32; 4],
}

pub struct TagInfo {
    pub name: String,
    pub commit_id: String,
}

/// Walk the full commit graph of a repository, tagging each commit with its branch name.
///
/// Walks each branch individually so commits know which branch they belong to.
/// Shared commits (reachable from multiple branches) get the non-"main" branch
/// if available — feature branches own their history.
pub fn traverse_repo(repo_path: &str) -> Result<CommitDag> {
    let repo = git2::Repository::open(repo_path)?;

    // ── Step 1: Discover branches ──
    let mut branches = Vec::new();
    let branch_iter = repo.branches(Some(git2::BranchType::Local))?;
    for branch in branch_iter.flatten() {
        let name = branch
            .0
            .name()
            .ok()
            .flatten()
            .unwrap_or("unknown")
            .to_string();
        let oid = branch.0.get().peel_to_commit().ok().map(|c| c.id());
        branches.push(BranchInfo {
            name,
            tip_commit: oid.map(|o| o.to_string()).unwrap_or_default(),
            color: [0.5, 0.5, 0.5, 1.0], // later overridden with golden-angle
        });
    }

    // Assign golden-angle colors to branches
    assign_branch_colors(&mut branches);

    // ── Step 2: Discover tags ──
    let mut tags = Vec::new();
    let tag_names = repo.tag_names(None)?;
    for tag_name in tag_names.iter().flatten() {
        if let Ok(obj) = repo.revparse_single(tag_name) {
            tags.push(TagInfo {
                name: tag_name.to_string(),
                commit_id: obj.id().to_string(),
            });
        }
    }

    // Build a quick lookup: tag commit_ids -> tag name
    let tag_map: HashMap<&str, &str> = tags
        .iter()
        .map(|t| (t.commit_id.as_str(), t.name.as_str()))
        .collect();

    // ── Step 3: Walk each branch, collect (commit_id, branch_name) pairs ──
    // Order branches so "main"/"master" is last — non-main wins on overlap
    let mut branch_order: Vec<&BranchInfo> = branches.iter().collect();
    branch_order.sort_by_key(|b| {
        if b.name == "main" || b.name == "master" { 1 } else { 0 }
    });

    // Collect {commit_id, branch_name} — later entries override earlier ones
    let mut commit_branch_map: HashMap<String, String> = HashMap::new();

    for branch in &branch_order {
        let oid = match Oid::from_str(&branch.tip_commit) {
            Ok(o) => o,
            Err(_) => continue,
        };
        let mut revwalk = match repo.revwalk() {
            Ok(w) => w,
            Err(_) => continue,
        };
        let _ = revwalk.push(oid);
        revwalk.set_sorting(git2::Sort::TIME).ok();

        for oid_result in revwalk {
            let commit_oid = match oid_result {
                Ok(o) => o,
                Err(_) => break,
            };
            let commit_id = commit_oid.to_string();
            // Non-main branches override main for shared commits
            commit_branch_map.insert(commit_id, branch.name.clone());
        }
    }

    // ── Step 4: Extract commit info for unique commits ──
    let mut commits = Vec::new();

    for (commit_id, branch_name) in &commit_branch_map {
        let oid = match Oid::from_str(commit_id) {
            Ok(o) => o,
            Err(_) => continue,
        };
        let commit = match repo.find_commit(oid) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let mut info = crate::git::commit::extract_commit_info(&repo, &commit)?;
        info.branch = branch_name.clone();

        // Mark as tag landmark if this commit has a tag
        if tag_map.contains_key(commit_id.as_str())
            && let Some(tag_name) = tag_map.get(commit_id.as_str())
        {
            info.tags.push(tag_name.to_string());
        }

        commits.push(info);
    }

    // Sort by timestamp ascending (oldest first) for consistent city layout
    commits.sort_by_key(|c| c.timestamp);

    Ok(CommitDag {
        commits,
        branches,
        tags,
    })
}

/// Assign each branch a unique neon color using golden-angle hue distribution.
/// Main/master gets the first (most saturated) color.
fn assign_branch_colors(branches: &mut [BranchInfo]) {
    let golden_angle = 137.508;

    // Sort so main/master is first
    branches.sort_by_key(|b| {
        if b.name == "main" || b.name == "master" {
            0
        } else {
            1
        }
    });

    for (i, branch) in branches.iter_mut().enumerate() {
        let hue = (i as f32 * golden_angle) % 360.0;
        let (r, g, b) = hsv_to_rgb(hue, 0.9, 0.7);
        branch.color = [r, g, b, 1.0];
    }
}

/// Convert HSV to RGB (all in 0.0–1.0 range)
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let hp = h / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    ((r1 + m).clamp(0.0, 1.0), (g1 + m).clamp(0.0, 1.0), (b1 + m).clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv_to_rgb_red() {
        let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0);
        assert!((r - 1.0).abs() < 0.001);
        assert!((g - 0.0).abs() < 0.001);
        assert!((b - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_hsv_to_rgb_green() {
        let (r, g, b) = hsv_to_rgb(120.0, 1.0, 1.0);
        assert!((r - 0.0).abs() < 0.001);
        assert!((g - 1.0).abs() < 0.001);
        assert!((b - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_hsv_to_rgb_blue() {
        let (r, g, b) = hsv_to_rgb(240.0, 1.0, 1.0);
        assert!((r - 0.0).abs() < 0.001);
        assert!((g - 0.0).abs() < 0.001);
        assert!((b - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_assign_branch_colors_main_first() {
        let mut branches = vec![
            BranchInfo { name: "feature".to_string(), tip_commit: "a".to_string(), color: [0.0, 0.0, 0.0, 1.0] },
            BranchInfo { name: "main".to_string(), tip_commit: "b".to_string(), color: [0.0, 0.0, 0.0, 1.0] },
        ];
        assign_branch_colors(&mut branches);
        assert_eq!(branches[0].name, "main");
    }
}