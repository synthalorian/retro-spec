mod cli;
mod city;
mod config;
mod git;
mod render;
mod ui;
mod export;
mod theme;
mod audio;

use clap::Parser;
use bevy::prelude::*;
use tracing_subscriber::EnvFilter;

/// App state — holds parsed data shared across Bevy systems
#[derive(Resource)]
struct AppState {
    commit_count: usize,
    branch_count: usize,
    author_count: usize,
    theme_name: String,
    city_meshes: city::builder::CityMeshes,
}

/// Hold commit data for HUD raycasting and info lookup
#[derive(Resource)]
struct CommitData {
    commits: Vec<git::commit::CommitInfo>,
}

/// Track which building the player is looking at (for diff pulse animation)
#[derive(Resource, Default)]
struct FocusedBuilding {
    pub commit_id: Option<String>,
}

fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    // ── Step 0: Load config file (retro-spec.toml) ──
    let cfg = config::load_config(std::path::Path::new("retro-spec.toml"));

    let mut args = cli::Cli::parse();

    // Merge config values with CLI args (CLI overrides config)
    if args.theme == "synthwave84" {
        // Check if config has a theme override
        if let Some(ref theme) = cfg.theme {
            args.theme = theme.clone();
        }
    }
    // Re-apply default if neither config nor CLI set it meaningfully
    // (CLI's default is "synthwave84", so if user didn't pass --theme we use config or stick with default)

    // Resolve repository path
    let repo_path = std::fs::canonicalize(&args.repo)
        .unwrap_or_else(|_| args.repo.clone())
        .to_string_lossy()
        .to_string();

    tracing::info!("Scanning repository: {}", repo_path);

    // ── Step 1: Walk the git DAG ──
    let dag = git::dag::traverse_repo(&repo_path)
        .map_err(|e| anyhow::anyhow!("Failed to parse git repo: {}", e))?;

    tracing::info!(
        "Found {} commits, {} branches, {} tags",
        dag.commits.len(),
        dag.branches.len(),
        dag.tags.len()
    );

    // ── Step 2: Stats-only mode ──
    if args.stats {
        print_stats(&dag, &repo_path);
        return Ok(());
    }

    // ── CI/CD mode: output JSON and optionally screenshot ──
    if args.ci {
        let mut author_set = std::collections::BTreeSet::new();
        let mut total_added = 0u64;
        let mut total_deleted = 0u64;
        let mut merges = 0;
        for c in &dag.commits {
            author_set.insert(c.author.as_str());
            total_added += c.lines_added as u64;
            total_deleted += c.lines_deleted as u64;
            if c.is_merge { merges += 1; }
        }
        let json = serde_json::json!({
            "repo": repo_path,
            "commits": dag.commits.len(),
            "authors": author_set.len(),
            "branches": dag.branches.len(),
            "tags": dag.tags.len(),
            "merges": merges,
            "lines_added": total_added,
            "lines_deleted": total_deleted,
            "net_change": total_added.abs_diff(total_deleted),
            "theme": args.theme,
        });
        println!("{}", serde_json::to_string_pretty(&json)?);
        // If CI mode without --screenshot, exit. With --screenshot, continue to render
        if args.screenshot.is_none() {
            return Ok(());
        }
    }

    // ── Step 3: Generate city plan ──
    let plan = city::planner::plan_city(&dag.commits)?;
    let meshes = city::builder::build_city(&plan);

    // ── Step 4: Count unique authors ──
    let mut authors = std::collections::HashSet::new();
    for c in &dag.commits {
        authors.insert(c.author.clone());
    }

    // ── Blame heat map analysis ──
    let blame_heatmap = git::blame::analyze_blame(&dag.commits);

    // ── Step 5: Set up app state ──
    let mut timeline_state = ui::timeline::TimelineState::new(dag.commits.len());
    timeline_state.init_from_commits(&dag.commits);

    let app_state = AppState {
        commit_count: dag.commits.len(),
        branch_count: dag.branches.len(),
        author_count: authors.len(),
        theme_name: args.theme.clone(),
        city_meshes: meshes,
    };

    let commit_data = CommitData {
        commits: dag.commits,
    };

    // ── Legend: extract author and directory colors ──
    let mut author_set: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for c in &commit_data.commits {
        author_set.insert(c.author.as_str());
    }

    let golden_angle = 137.508;
    let legend_authors: Vec<(String, [f32; 4])> = author_set
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let hue = (i as f32 * golden_angle) % 360.0;
            let (r, g, b) = hsv_to_rgb(hue, 0.8, 0.9);
            (name.to_string(), [r, g, b, 1.0])
        })
        .collect();

    let legend_dirs: Vec<(String, [f32; 4])> = plan
        .districts
        .iter()
        .map(|d| (d.name.clone(), d.color))
        .collect();

    let mut legend_state = ui::legend::LegendState::new();
    legend_state.authors = legend_authors;
    legend_state.directories = legend_dirs;

    // ── Step 6: Launch Bevy ──
    let mut app = App::new();

    app.insert_resource(blame_heatmap);
    app.insert_resource(FocusedBuilding::default());
    app.insert_resource(ui::hud::HudState::new(
        app_state.commit_count,
        app_state.branch_count,
        app_state.author_count,
    ));
    app.insert_resource(app_state);
    app.insert_resource(commit_data);
    app.insert_resource(timeline_state);
    app.insert_resource(legend_state);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "RetroSpec — Walk Your Code".to_string(),
            resolution: bevy::window::WindowResolution::new(1920.0, 1080.0),
            ..Default::default()
        }),
        ..Default::default()
    }));

    // ── Screenshot (only if --screenshot flag is set) ──
    if args.screenshot.is_some() {
        let screenshot_path = args.screenshot.clone().unwrap().to_string_lossy().to_string();
        app.insert_resource(export::screenshot::ScreenshotPath(screenshot_path));
        app.add_systems(Update, export::screenshot::capture_screenshot);
    }

    // ── Video export (only if --export flag is set) ──
    if args.export.is_some() {
        let export_dir = args.export.clone().unwrap().to_string_lossy().to_string();
        // Create directory
        let _ = std::fs::create_dir_all(&export_dir);
        app.insert_resource(export::video::ExportState::new(
            export_dir,
            args.duration,
            30,
        ));
        app.add_systems(Update, export::video::export_flythrough);
    }

    app.add_systems(Startup, setup_scene);
    app.add_systems(Startup, ui::setup_hud);
    app.add_systems(Startup, ui::setup_timeline);
    app.add_systems(Startup, ui::setup_legend);
    app.add_systems(Startup, audio::setup_audio);
    app.add_systems(Update, render::fly_camera_controls);
    app.add_systems(Update, ui::hud_system);
    app.add_systems(Update, ui::timeline_interaction);
    app.add_systems(Update, ui::update_building_visibility);
    app.add_systems(Update, ui::toggle_legend_system);
    app.add_systems(Update, render::rotate_tag_beacons);
    app.add_systems(Update, render::animate_particles);
    app.add_systems(Update, render::apply_lod);
    app.add_systems(Update, render::animate_focused_building);

    app.run();

    Ok(())
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    state: Res<AppState>,
    blame_heatmap: Res<crate::git::blame::BlameHeatmap>,
    commit_data: Res<CommitData>,
) {
    // Apply theme
    let theme = theme::get_theme(&state.theme_name);

    // Terrain
    render::terrain::spawn_terrain(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgba(theme.grid_color[0], theme.grid_color[1], theme.grid_color[2], theme.grid_color[3]),
        Color::srgba(theme.ground_color[0], theme.ground_color[1], theme.ground_color[2], theme.ground_color[3]),
    );

    // Buildings
    render::buildings::spawn_buildings(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut images,
        &state.city_meshes.buildings,
        Some(&blame_heatmap),
        Some(&commit_data),
    );

    // Streets
    render::streets::spawn_streets(
        &mut commands,
        &mut meshes,
        &mut materials,
        &state.city_meshes.streets,
    );

    // Merge plazas
    render::streets::spawn_merge_plazas(
        &mut commands,
        &mut meshes,
        &mut materials,
        &state.city_meshes.plazas,
    );

    // Skybridges
    render::streets::spawn_skybridges(
        &mut commands,
        &mut meshes,
        &mut materials,
        &state.city_meshes.skybridges,
    );

    // Districts
    render::districts::spawn_districts(
        &mut commands,
        &mut meshes,
        &mut materials,
        &state.city_meshes.districts,
    );

    // Lighting
    render::lighting::setup_lighting(&mut commands, &theme);

    // Camera
    render::camera::spawn_camera(&mut commands);

    // Ambient particles
    render::particles::spawn_particles(&mut commands, &mut meshes, &mut materials);
}

fn print_stats(dag: &git::dag::CommitDag, repo_path: &str) {
    let mut authors = std::collections::HashSet::new();
    let mut total_added = 0u64;
    let mut total_deleted = 0u64;
    let mut merges = 0;

    for c in &dag.commits {
        authors.insert(c.author.clone());
        total_added += c.lines_added as u64;
        total_deleted += c.lines_deleted as u64;
        if c.is_merge {
            merges += 1;
        }
    }

    println!("═══ RetroSpec — Repo Stats ═══");
    println!("  Repository:     {}", repo_path);
    println!("  Commits:        {}", dag.commits.len());
    println!("  Authors:        {}", authors.len());
    println!("  Branches:       {}", dag.branches.len());
    println!("  Tags:           {}", dag.tags.len());
    println!("  Merges:         {}", merges);
    println!("  Lines added:    {}", total_added);
    println!("  Lines deleted:  {}", total_deleted);
    println!("  Net change:     {}", if total_added > total_deleted {
        format!("+{}", total_added - total_deleted)
    } else {
        format!("-{}", total_deleted - total_added)
    });
    if let Some(earliest) = dag.commits.last() {
        println!("  First commit:   {} by {}",
            chrono::DateTime::from_timestamp(earliest.timestamp, 0)
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
            earliest.author
        );
    }
    if let Some(latest) = dag.commits.first() {
        println!("  Latest commit:  {} by {}",
            chrono::DateTime::from_timestamp(latest.timestamp, 0)
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
            latest.author
        );
    }
    println!("═══");
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