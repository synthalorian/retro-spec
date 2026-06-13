use std::collections::HashMap;

/// Map a set of file paths to districts with color-coding
#[allow(dead_code)]
pub fn map_districts(paths: &[String]) -> Vec<DistrictInfo> {
    // Group by top-level directory
    let mut dir_counts: HashMap<String, usize> = HashMap::new();
    for path in paths {
        // Extract first path segment
        let top_dir = path
            .split('/')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or("root")
            .to_string();
        *dir_counts.entry(top_dir).or_insert(0) += 1;
    }

    if dir_counts.is_empty() {
        return vec![];
    }

    // Sort by frequency (most impactful first)
    let mut sorted: Vec<(&String, &usize)> = dir_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    // Assign colors via golden angle
    let golden_angle = 137.508;
    sorted
        .into_iter()
        .enumerate()
        .map(|(i, (path, &count))| {
            let hue = (i as f32 * golden_angle + 200.0) % 360.0; // offset from author hues
            let (r, g, b) = hsv_to_rgb(hue, 0.6, 0.5); // more muted than author colors
            (
                path.clone(),
                DistrictInfo {
                    path: path.clone(),
                    color: [r, g, b, 0.15], // very transparent
                    commit_count: count,
                },
            )
        })
        .map(|(_, info)| info)
        .collect()
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

    (
        (r1 + m).clamp(0.0, 1.0),
        (g1 + m).clamp(0.0, 1.0),
        (b1 + m).clamp(0.0, 1.0),
    )
}

/// Filesystem directory → city district mapping
#[allow(dead_code)]
pub struct DistrictInfo {
    pub path: String,
    pub color: [f32; 4],
    pub commit_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_districts_basic() {
        let paths = vec![
            "src/main.rs".to_string(),
            "src/lib.rs".to_string(),
            "docs/readme.md".to_string(),
            "assets/icons/icon.png".to_string(),
            "Cargo.toml".to_string(),
        ];
        let districts = map_districts(&paths);
        assert_eq!(districts.len(), 4);
        assert!(districts.iter().any(|d| d.path == "src"));
        assert!(districts.iter().any(|d| d.path == "docs"));
        assert!(districts.iter().any(|d| d.path == "assets"));
        assert!(districts.iter().any(|d| d.path == "Cargo.toml"));
    }
}