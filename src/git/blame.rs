use crate::git::commit::CommitInfo;
use bevy::prelude::*;
use std::collections::HashMap;

/// Blame analysis — which files contributed to change activity
#[derive(Resource)]
pub struct BlameHeatmap {
    pub hotspots: Vec<FileHotspot>,
}

pub struct AuthorContribution {
    pub commit_count: usize,
}

pub struct FileHotspot {
    pub path: String,
    pub heat: f32, // 0.0 to 1.0 — how much change activity
}

/// Analyze blame distribution across a set of commits.
/// Aggregates per-author stats and computes file hotspot heat values.
pub fn analyze_blame(commits: &[CommitInfo]) -> BlameHeatmap {
    // ── Per-author aggregation ──
    let mut author_map: HashMap<String, (usize, u32, u32)> = HashMap::new();

    // ── Per-file change counts ──
    let mut file_changes: HashMap<String, usize> = HashMap::new();

    for c in commits {
        // Author stats
        let entry = author_map
            .entry(c.author.clone())
            .or_insert((0, 0, 0));
        entry.0 += 1;                // commit count
        entry.1 += c.lines_added;    // lines added
        entry.2 += c.lines_deleted;  // lines deleted

        // File change counts
        for f in &c.files {
            *file_changes.entry(f.clone()).or_insert(0) += 1;
        }
    }

    // Convert author map to sorted vec
    let mut authors: Vec<AuthorContribution> = author_map
        .into_iter()
        .map(|(_name, (commit_count, _lines_added, _lines_deleted))| AuthorContribution {
            commit_count,
        })
        .collect();
    authors.sort_by_key(|b| std::cmp::Reverse(b.commit_count));

    // Compute file hotspot heat (relative change activity)
    let max_changes = file_changes.values().copied().max().unwrap_or(1);
    let mut hotspots: Vec<FileHotspot> = file_changes
        .into_iter()
        .map(|(path, count)| FileHotspot {
            path,
            heat: if max_changes > 0 {
                count as f32 / max_changes as f32
            } else {
                0.0
            },
        })
        .collect();
    hotspots.sort_by(|a, b| b.heat.partial_cmp(&a.heat).unwrap_or(std::cmp::Ordering::Equal));

    BlameHeatmap { hotspots }
}