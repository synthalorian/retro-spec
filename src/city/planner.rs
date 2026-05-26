use std::collections::HashMap;

/// Planned city layout — street grid + lot assignments
pub struct CityPlan {
    pub streets: Vec<Street>,
    pub lots: Vec<Lot>,
    pub districts: Vec<District>,
    pub plazas: Vec<MergePlaza>,
    pub skybridges: Vec<Skybridge>,
}

pub struct Street {
    pub name: String,
    pub start: (f32, f32),
    pub end: (f32, f32),
    pub width: f32,
    pub color: [f32; 4],
}

pub struct Lot {
    pub position: (f32, f32),
    pub width: f32,
    pub depth: f32,
    pub height: f32,
    pub color: [f32; 4],
    pub commit_id: String,
    pub is_tagged: bool,
    pub district: String,
    pub timestamp: i64,
}

pub struct District {
    pub name: String,
    pub bounds: (f32, f32, f32, f32),
    pub color: [f32; 4],
}

/// A glowing intersection where two branch boulevards meet
pub struct MergePlaza {
    pub position: (f32, f32),
    pub radius: f32,
    pub color: [f32; 4],
    pub commit_id: String,
    pub branches: Vec<String>,
}

/// A glass skybridge connecting matching commits across branch boulevards
pub struct Skybridge {
    pub start: (f32, f32),
    pub end: (f32, f32),
    pub height: f32,
    pub color: [f32; 4],
    pub message: String,
}

/// Constants for city layout
const STREET_WIDTH: f32 = 4.0;
const STREET_SPACING: f32 = 10.0; // gap between parallel branch streets
const LOT_WIDTH: f32 = 2.0;
const LOT_DEPTH: f32 = 2.0;
const BUILDING_MIN_HEIGHT: f32 = 0.5;
const BUILDING_MAX_HEIGHT: f32 = 20.0;
const BASE_SPACING: f32 = 3.0; // minimum gap between buildings (no scaling)
const CITY_HALF_WIDTH: f32 = 80.0; // half the city grid size

/// Generate a city plan from parsed git commit data.
pub fn plan_city(commits: &[crate::git::commit::CommitInfo]) -> anyhow::Result<CityPlan> {
    if commits.is_empty() {
        return Ok(CityPlan {
            streets: vec![],
            lots: vec![],
            districts: vec![],
            plazas: vec![],
            skybridges: vec![],
        });
    }

    // ── Phase 1: Assign author colors using golden-angle distribution ──
    let author_colors = assign_author_colors(commits);

    // ── Phase 2: Group commits by branch ──
    let branch_groups = group_by_branch(commits);

    // ── Phase 3: Build streets — one per branch, parallel in Z ──
    let streets = build_streets(&branch_groups);

    // ── Phase 4: Assign lots — temporal spacing along each branch ──
    let lots = assign_lots(commits, &branch_groups, &streets, &author_colors);

    // ── Phase 5: Districts — color-coded neighborhoods ──
    let districts = build_districts(commits, &lots);

    // ── Phase 6: Merge intersection plazas — glowing crossroads ──
    let plazas = build_merge_plazas(commits, &branch_groups, &streets);

    // ── Phase 7: Skybridges — glass connectors linking matching commits across branches ──
    let skybridges = build_skybridges(commits, &lots);

    Ok(CityPlan {
        streets,
        lots,
        districts,
        plazas,
        skybridges,
    })
}

/// Assign each unique author a distinct neon color using golden-angle hue distribution.
fn assign_author_colors(commits: &[crate::git::commit::CommitInfo]) -> HashMap<String, [f32; 4]> {
    let mut authors: Vec<&str> = commits.iter().map(|c| c.author.as_str()).collect();
    authors.sort();
    authors.dedup();

    let golden_angle = 137.508; // degrees — maximizes hue separation
    let saturation = 0.8;
    let value = 0.9;
    let alpha = 1.0;

    authors
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            let hue = (i as f32 * golden_angle) % 360.0;
            let (r, g, b) = hsv_to_rgb(hue, saturation, value);
            (name.to_string(), [r, g, b, alpha])
        })
        .collect()
}

/// Group commits by their branch name.
struct BranchGroup<'a> {
    name: &'a str,
    commits: Vec<&'a crate::git::commit::CommitInfo>,
}

fn group_by_branch(commits: &[crate::git::commit::CommitInfo]) -> Vec<BranchGroup> {
    let mut groups: HashMap<&str, Vec<&crate::git::commit::CommitInfo>> = HashMap::new();
    for c in commits {
        let branch = if c.branch.is_empty() { "main" } else { &c.branch };
        groups.entry(branch).or_default().push(c);
    }

    // Sort groups: main/master first, then alphabetically
    let mut result: Vec<BranchGroup> = groups
        .into_iter()
        .map(|(name, commits)| BranchGroup { name, commits })
        .collect();
    result.sort_by_key(|g| {
        if g.name == "main" || g.name == "master" {
            (0, "")
        } else {
            (1, g.name)
        }
    });

    result
}

/// Build parallel streets — one per branch, offset along Z axis.
/// Each street gets a unique golden-angle color and width proportional to commit count.
fn build_streets(branch_groups: &[BranchGroup]) -> Vec<Street> {
    let golden_angle = 137.508;
    let max_commits = branch_groups
        .iter()
        .map(|g| g.commits.len())
        .max()
        .unwrap_or(1)
        .max(1);

    branch_groups
        .iter()
        .enumerate()
        .map(|(i, group)| {
            let z_offset = i as f32 * (STREET_WIDTH + STREET_SPACING) - center_z(branch_groups.len());

            // Golden-angle color for this branch
            let hue = (i as f32 * golden_angle) % 360.0;
            let (r, g, b) = hsv_to_rgb(hue, 0.9, 0.7);

            // Street width proportional to commit count (min 2.0, max 8.0)
            let width_ratio = group.commits.len() as f32 / max_commits as f32;
            let street_width = 2.0 + width_ratio * 6.0;

            Street {
                name: group.name.to_string(),
                start: (-CITY_HALF_WIDTH, z_offset),
                end: (CITY_HALF_WIDTH, z_offset),
                width: street_width,
                color: [r, g, b, 0.8],
            }
        })
        .collect()
}

/// Center the city grid on Z=0
fn center_z(num_streets: usize) -> f32 {
    if num_streets <= 1 {
        0.0
    } else {
        (num_streets - 1) as f32 * (STREET_WIDTH + STREET_SPACING) / 2.0
    }
}

/// Assign lots — commits placed along their branch's street with temporal spacing.
///
/// Spacing = log(time_delta_in_days + 1) * scale
/// This keeps close commits tight and distant ones apart without infinite spread.
fn assign_lots(
    commits: &[crate::git::commit::CommitInfo],
    branch_groups: &[BranchGroup],
    streets: &[Street],
    author_colors: &HashMap<String, [f32; 4]>,
) -> Vec<Lot> {
    // Map branch name → street index
    let branch_to_street: HashMap<&str, usize> = streets
        .iter()
        .enumerate()
        .map(|(i, s)| (s.name.as_str(), i))
        .collect();

    // Pre-compute commit size statistics for height scaling
    let sizes: Vec<f32> = commits
        .iter()
        .map(|c| (c.lines_added + c.lines_deleted) as f32)
        .collect();
    let max_size = sizes.iter().cloned().fold(0.0f32, f32::max).max(1.0);
    let _mean_size = sizes.iter().sum::<f32>() / sizes.len().max(1) as f32;

    let mut lots = Vec::new();

    for group in branch_groups {
        let street_idx = branch_to_street.get(group.name).copied().unwrap_or(0);
        let street = &streets[street_idx];
        let z_pos = street.start.1;

        let group_commits = &group.commits;

        // Sort commits in this branch by timestamp
        let mut sorted: Vec<&&crate::git::commit::CommitInfo> = group_commits.iter().collect();
        sorted.sort_by_key(|c| c.timestamp);

        if sorted.is_empty() {
            continue;
        }

        let first_ts = sorted.first().unwrap().timestamp;
        let last_ts = sorted.last().unwrap().timestamp;
        let _total_span = (last_ts - first_ts).max(1) as f32; // seconds

        let mut x_pos = -CITY_HALF_WIDTH + BASE_SPACING;

        for (ci, c) in sorted.iter().enumerate() {
            // Temporal spacing: use days between consecutive commits
            let gap = if ci == 0 {
                BASE_SPACING // first commit on this branch starts at origin
            } else {
                let prev = sorted[ci - 1];
                let delta_seconds = (c.timestamp - prev.timestamp).max(1) as f32;
                let delta_days = delta_seconds / 86400.0;
                // Log scale: log(1) = 0, log(365) ≈ 5.9, log(3650) ≈ 8.2
                BASE_SPACING + (delta_days + 1.0).ln() * 1.5
            };
            x_pos += gap;

            // Compute height from commit size
            let size = (c.lines_added + c.lines_deleted) as f32;
            let height_ratio = if max_size == 0.0 {
                0.1
            } else {
                size / max_size
            };
            // Quadratic curve: small commits visible, big commits tower
            let height =
                BUILDING_MIN_HEIGHT + (BUILDING_MAX_HEIGHT - BUILDING_MIN_HEIGHT) * height_ratio.sqrt();
            let height = height.clamp(BUILDING_MIN_HEIGHT, BUILDING_MAX_HEIGHT);

            // Color from author
            let color = author_colors
                .get(c.author.as_str())
                .copied()
                .unwrap_or([0.2, 0.6, 1.0, 1.0]);

            // Tagged commits get wider base and a gold shimmer
            let (width, is_tagged) = if c.is_tagged {
                (LOT_WIDTH * 1.5, true)
            } else {
                (LOT_WIDTH, false)
            };

            lots.push(Lot {
                position: (x_pos, z_pos),
                width,
                depth: LOT_DEPTH,
                height,
                color,
                commit_id: c.id.clone(),
                is_tagged,
                district: extract_district(c),
                timestamp: c.timestamp,
            });
        }
    }

    lots
}

/// Build districts — group by top-level directory paths
fn build_districts(
    _commits: &[crate::git::commit::CommitInfo],
    lots: &[Lot],
) -> Vec<District> {
    // Group lots by district name
    let mut district_lots: HashMap<&str, Vec<&Lot>> = HashMap::new();
    for lot in lots {
        let dname = if lot.district.is_empty() { "root" } else { &lot.district };
        district_lots.entry(dname).or_default().push(lot);
    }

    if district_lots.is_empty() {
        return vec![];
    }

    // Assign colors via golden-angle (offset from author colors)
    let golden_angle = 137.508;
    let mut sorted_names: Vec<&&str> = district_lots.keys().collect();
    sorted_names.sort();

    sorted_names
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            let lots = &district_lots[*name];
            let hue = (i as f32 * golden_angle + 200.0) % 360.0;
            let (r, g, b) = hsv_to_rgb(hue, 0.6, 0.5);

            // Compute bounding box of lots in this district
            let min_x = lots.iter().map(|l| l.position.0).fold(f32::MAX, f32::min);
            let max_x = lots.iter().map(|l| l.position.0).fold(f32::MIN, f32::max);
            let min_z = lots.iter().map(|l| l.position.1).fold(f32::MAX, f32::min);
            let max_z = lots.iter().map(|l| l.position.1).fold(f32::MIN, f32::max);

            District {
                name: name.to_string(),
                bounds: (min_x - 5.0, min_z - 5.0, max_x + 5.0, max_z + 5.0),
                color: [r, g, b, 0.12], // very transparent ground tint
            }
        })
        .collect()
}

/// Build merge intersection plazas — glowing crossroads where branch boulevards meet.
///
/// For each merge commit, we find its parent branches, compute the intersection
/// point between those branch streets, and spawn a glowing plaza marker.
fn build_merge_plazas(
    commits: &[crate::git::commit::CommitInfo],
    branch_groups: &[BranchGroup],
    streets: &[Street],
) -> Vec<MergePlaza> {
    // Build commit → branch name map
    let mut commit_to_branch: HashMap<&str, &str> = HashMap::new();
    for group in branch_groups {
        for c in &group.commits {
            commit_to_branch.insert(c.id.as_str(), group.name);
        }
    }

    // Build branch name → street Z position map
    let branch_z: HashMap<&str, f32> = streets
        .iter()
        .map(|s| (s.name.as_str(), s.start.1)) // all streets parallel in Z
        .collect();

    let mut plazas = Vec::new();

    // Sort commits by timestamp for consistent layout (matches assign_lots order)
    let mut sorted: Vec<&crate::git::commit::CommitInfo> = commits.iter().collect();
    sorted.sort_by_key(|c| c.timestamp);
    sorted.dedup_by_key(|c| &c.id);

    // Recreate the X-position tracking per branch to match lot positions
    let mut branch_x_positions: HashMap<&str, f32> = HashMap::new();
    for c in &sorted {
        let branch = if c.branch.is_empty() { "main" } else { &c.branch };

        if !c.is_merge {
            let x_pos = branch_x_positions.entry(branch).or_insert(-CITY_HALF_WIDTH + BASE_SPACING);
            let gap = BASE_SPACING; // simplified — matches main spacing logic
            *x_pos += gap;
        }
    }

    // Reset branch X tracking for merge plaza extraction
    let mut branch_x: HashMap<String, f32> = HashMap::new();

    for c in &sorted {
        let branch = if c.branch.is_empty() { "main" } else { &c.branch };
        let x_entry = branch_x.entry(branch.to_string()).or_insert(-CITY_HALF_WIDTH + BASE_SPACING);

        let prev_x = *x_entry;

        // Compute spacing matching assign_lots
        let gap = if *x_entry != -CITY_HALF_WIDTH + BASE_SPACING {
            // We don't have the exact previous timestamp here, use BASE_SPACING
            BASE_SPACING
        } else {
            BASE_SPACING
        };
        *x_entry += gap;

        if !c.is_merge {
            continue;
        }

        // This is a merge commit — find parent branches for intersection
        let parent_id = c.parents.first().map(|s| s.as_str()).unwrap_or("");
        let parent_branch = commit_to_branch.get(parent_id).copied();
        let own_branch = branch;

        if let Some(pb) = parent_branch {
            if pb != own_branch {
                // Different branches — create intersection plaza
                let z_self = branch_z.get(own_branch).copied().unwrap_or(0.0);
                let z_parent = branch_z.get(pb).copied().unwrap_or(0.0);
                let z_mid = (z_self + z_parent) / 2.0;
                let plaza_z = z_mid;

                // Plaza radius proportional to spacing between streets
                let radius = (z_self - z_parent).abs().max(STREET_SPACING) / 2.0;

                // Gold-tinted color
                let plaza_color = [1.0, 0.7, 0.2, 0.8];

                plazas.push(MergePlaza {
                    position: (prev_x, plaza_z),
                    radius: radius.max(2.0),
                    color: plaza_color,
                    commit_id: c.id.clone(),
                    branches: vec![own_branch.to_string(), pb.to_string()],
                });
            }
        }
    }

    plazas
}

/// Build skybridges — glass connectors linking matching commits across different branches.
///
/// Groups commits by their message subject (first line). For each subject that appears
/// on different branches, creates a skybridge between the matching lots.
fn build_skybridges(
    commits: &[crate::git::commit::CommitInfo],
    lots: &[Lot],
) -> Vec<Skybridge> {
    use std::collections::{HashMap, HashSet};

    // Group commits by their message first line (subject)
    let mut subject_commits: HashMap<String, Vec<&crate::git::commit::CommitInfo>> = HashMap::new();
    for c in commits {
        let subject = c.message.lines().next().unwrap_or("").to_string();
        if !subject.is_empty() {
            subject_commits.entry(subject).or_default().push(c);
        }
    }

    // Map commit IDs to their lots for fast lookup
    let commit_to_lot: HashMap<&str, &Lot> = lots.iter().map(|l| (l.commit_id.as_str(), l)).collect();

    let mut skybridges = Vec::new();

    for (message, matching_commits) in &subject_commits {
        // Only create skybridges if the same subject appears in different branches
        let branches: HashSet<&str> = matching_commits
            .iter()
            .map(|c| if c.branch.is_empty() { "main" } else { &c.branch })
            .collect();

        if branches.len() < 2 {
            continue;
        }

        // Collect all lots for these matching commits
        let mut bridge_lots: Vec<&Lot> = matching_commits
            .iter()
            .filter_map(|c| commit_to_lot.get(c.id.as_str()).copied())
            .collect();

        if bridge_lots.len() < 2 {
            continue;
        }

        // Sort by timestamp for consistent pairing
        bridge_lots.sort_by_key(|l| l.timestamp);

        // Connect lots in pairs (first with second, third with fourth, etc.)
        for chunk in bridge_lots.chunks(2) {
            if chunk.len() < 2 {
                continue;
            }
            let lot1 = chunk[0];
            let lot2 = chunk[1];

            // Average height of the two buildings
            let avg_height = (lot1.height + lot2.height) / 2.0;

            skybridges.push(Skybridge {
                start: (lot1.position.0, lot1.position.1),
                end: (lot2.position.0, lot2.position.1),
                height: avg_height,
                color: [0.5, 0.7, 1.0, 0.4], // glass/cyan
                message: message.clone(),
            });
        }
    }

    skybridges
}

/// Extract the top-level directory from a commit's files
fn extract_district(commit: &crate::git::commit::CommitInfo) -> String {
    commit
        .files
        .first()
        .map(|f| {
            f.split('/')
                .next()
                .filter(|s| !s.is_empty())
                .unwrap_or("root")
                .to_string()
        })
        .unwrap_or_default()
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
