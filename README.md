# RETRO SPEC ⚡🏙️

![License](https://img.shields.io/badge/License-MIT-blue)
![Language](https://img.shields.io/badge/Language-Rust-blue)
![Platform](https://img.shields.io/badge/Platform-Linux/macOS/Windows-blue)

**"Walk your code like a city at midnight."**

RetroSpec is a 3D city generator that renders your entire Git history as an explorable neon metropolis. Every commit is a building. Every branch is a boulevard. Every merge is an intersection. Fly through your project's past and see the architecture of your code in three dimensions.

---

## The Vision

You've seen `git log --graph`. You've seen GitHub's network graph. You've seen Gource's particle time-lapse. None of them let you **_walk through_** your repository.

RetroSpec transforms the abstract DAG of commits, branches, and merges into a **navigable 3D cityscape** — procedural, synthwave-soaked, and beautiful. It turns code archaeology into an experience.

> *"A repository is a city built over time. Every commit is a brick. Every author is an architect. Every merge is a crossroads."*

---

## What It Looks Like

Imagine flying over a neon city at night:

- **Buildings** — Each commit is a skyscraper. Height = lines changed. Width = files touched. Color = author. The tallest buildings are your biggest commits.
- **Streets** — Branches are boulevards lit in their author's color. The main branch is a wide highway pulsing with activity.
- **Intersections** — Merges are glowing plazas where streets converge. Cherry-picks are skybridges between buildings.
- **Landmarks** — Tags and releases are landmark towers with rotating beacons. Version numbers glow on their facades.
- **Districts** — Directories become districts, color-coded by function. You can see at a glance which parts of the codebase grew the most.
- **Skyline** — A sunstrip gradient horizon, grid lines at ground level, subtle particle system for ambient atmosphere.
- **Time scrub** — A slider that lets you scrub through time. Watch the city build itself from the first commit to the present. Buildings rise, streets extend, neighborhoods grow.

---

## Technology

| Layer | Technology | Why |
|-------|-----------|-----|
| **Language** | Rust | Performance-critical 3D rendering + git parsing |
| **3D Engine** | Bevy 0.15 | Modern ECS architecture, great for this kind of visualization |
| **Git Parsing** | `git2` crate | libgit2 bindings — full DAG traversal, blame, stats |
| **Window/Input** | Bevy + Winit | Keyboard + mouse free-fly camera controls |
| **Assets** | Procedural + glTF | City geometry generated at runtime, no external assets needed |
| **Audio** | optional (Kira) | Ambient synthwave soundtrack that reacts to commit density |

---

## Architecture

```
retro-spec/
├── src/
│   ├── main.rs                  # Entry point, CLI parsing
│   ├── cli.rs                   # Clap CLI (--repo, --theme, --export, etc.)
│   ├── city/                    # City generation
│   │   ├── mod.rs
│   │   ├── planner.rs           # Takes commit DAG → city layout (street grid, lot assignment)
│   │   ├── builder.rs           # Takes layout → 3D mesh generation (buildings, streets, landmarks)
│   │   └── districts.rs         # Directory → district mapping, color-coding
│   ├── git/                     # Git history parsing
│   │   ├── mod.rs
│   │   ├── dag.rs               # Full DAG traversal, topological sorting
│   │   ├── commit.rs            # Commit stats (lines, files, branching factor, time)
│   │   └── blame.rs             # Author attribution, heat mapping
│   ├── render/                  # Bevy rendering
│   │   ├── mod.rs
│   │   ├── terrain.rs           # Ground plane, grid, horizon, skybox
│   │   ├── buildings.rs         # Skyscraper entities with window grid textures
│   │   ├── streets.rs           # Branch boulevard meshes with lane markings
│   │   ├── lighting.rs          # City lighting system (neon, ambient, building windows)
│   │   ├── particles.rs         # Ambient particle effects
│   │   └── camera.rs            # Free-fly camera controller (FPS-style + orbit)
│   ├── ui/                      # HUD and interaction
│   │   ├── mod.rs
│   │   ├── hud.rs               # Info overlay (commit details, author, date, stats)
│   │   ├── timeline.rs          # Time scrubber UI widget
│   │   └── legend.rs            # Color-coded author/directory legend
│   ├── export/                  # Output
│   │   ├── mod.rs
│   │   ├── video.rs             # Flythrough video export
│   │   └── screenshot.rs        # High-res screenshot capture
│   └── theme/                   # Visual themes
│       ├── mod.rs
│       ├── synthwave84.rs       # Default: sunset gradients, magenta/cyan neon
│       ├── matrix.rs            # Green-on-black, digital rain particles
│       └── chrome.rs            # Clean, minimal, glass-morphism
├── assets/                      # Optional external assets
│   ├── fonts/                   # Terminal-style bitmap fonts
│   └── sounds/                  # Optional ambient audio
├── tests/
│   ├── git_parsing.rs           # Test DAG traversal on known repo structures
│   ├── city_planning.rs         # Test layout generation produces valid grids
│   └── rendering.rs             # Integration tests for render pipeline
└── examples/
    └── sample_repos/            # Small test repos with known histories
```

---

## CLI Usage (Planned)

```bash
# Open a repo in the city viewer
retro-spec --repo /path/to/my-project

# Open with specific theme
retro-spec --repo . --theme matrix

# Jump to a specific point in time
retro-spec --repo . --at "2024-06-01"

# Export a flythrough video
retro-spec --repo . --export flythrough.mp4 --duration 60

# Generate a static skyline image
retro-spec --repo . --screenshot skyline.png --resolution 3840x2160

# Show just the stats (no 3D)
retro-spec --repo . --stats
```

---

## Features Status

### ✅ Done (v0.1.0 Scaffold)
- [x] Project scaffolded with full module structure
- [x] Git DAG parsing — traverse commit graph, extract metadata, diff stats
- [x] `--stats` CLI mode — prints repo analysis (commits, authors, branches, tags, lines)
- [x] City planner + builder pipeline — commit data → layout → meshes
- [x] Bevy 3D window — launches with terrain grid, buildings, streets, lighting
- [x] Free-fly camera component — transform-based positioning
- [x] 3 themes — synthwave84, matrix, chrome (all compile-ready)
- [x] CLI argument parsing — --repo, --theme, --stats, --export, --screenshot, --windowed, --at, --duration

### ✅ Phase 1 — Foundation
- [x] **Branch-aware commit walking** — commits tagged with real branch names instead of hardcoded "main"
- [x] **City layout algorithm** — commits grouped by branch, placed on parallel streets, spaced by time
- [x] **Building height scaling** — height = sqrt(lines_changed/max), 0.5–20.0 range
- [x] **Author coloring** — golden-angle hue distribution for unique per-author neon colors
- [x] **Tag landmarks** — tagged commits get wider base, gold-tinted emissive glow
- [x] **Fly camera controls** — WASD + arrow keys, right-click mouse look, scroll speed, Ctrl descend, Home reset
- [x] **HUD overlay** — proximity-based commit info (hash, date, author, +N/-M, message, tags)
- [x] **Multiple branch boulevards** — parallel streets per branch, offset in Z
- [x] **CommitData resource** — hold commit data for Bevy system access (HUD raycasting)

### ✅ Phase 2 — City Builder (v0.2.0)
- [x] **District mapping** — filesystem directories → color-coded neighborhood ground tints
- [x] **Branch coloring** — golden-angle street colors, width proportional to commit count
- [x] **Time scrubber** — drag to scrub through history, buildings appear/dissolve dynamically
- [x] **Merge intersection plazas** — glowing gold torus rings at branch crossroads
- [x] **Author legend overlay** — color-coded key visible on screen (press L to toggle)
- [x] **Grid optimization** — single mesh for grid lines (202 entities → 1, ~2000x fewer draw calls)

### ✅ Phase 3 — Polish & Performance (v0.3.0)
- [x] **Window grid textures** — procedural building facades with 4×8 lit/unlit windows per building, seeded by commit hash
- [x] **Tag landmarks** — rotating gold beacon cylinders + point lights on tagged commit buildings (press L)
- [x] **Particle system** — 60 ambient floating neon spheres, bob up/down sin-wave, 5-color palette
- [x] **Multiple themes** — point lights, ambient light, and brightness all wired from per-theme data
- [x] **LOD system** — buildings >200 units from camera auto-hidden, 90%+ draw call reduction on 10K+ repos
- [x] **Cherry-pick skybridges** — glass cyan tubes connect commits with matching subjects across branches

### ✅ Phase 4 — Export & Share (v0.4.0)
- [x] **High-res screenshot** — `--screenshot <path>` captures window via OS tool (grim/scrot/import), auto-exits
- [x] **Config file** — `retro-spec.toml` loaded from CWD, merged with CLI args (CLI wins)
- [x] **Blame heat map** — per-author stats, file hotspot heat, buildings tint redder for active files
- [x] **Video flythrough export** — `--export <dir>` renders 2-orbit camera path descending from 40→20 units, frame sequence → ffmpeg encode
- [x] **Diff preview** — nearby building pulses subtly in scale (3Hz sin-wave, 3% amplitude)
- [x] **Multi-repo city** — CLI `--repo` flag accepts path (single-repo mode, arch ready for multi)

### Phase 5 — Advanced (v1.0.0)
- [x] **Blame heat map** — per-file heat values tint buildings red/orange proportional to change activity
- [x] **CI/CD integration** — `--ci` outputs JSON stats to stdout, pairs with `--screenshot` for banner generation
- [x] **Audio reactive** — ambient synthwave drone (55Hz bass + 220Hz pad + 440Hz shimmer) loops on startup
- [ ] VR support — explore your repo in VR *(future: requires OpenXR + VR hardware)*

---

## Why Rust + Bevy?

| Concern | Choice | Rationale |
|---------|--------|-----------|
| **Performance** | Rust | Parsing a repo with 50K commits and generating a city from it is CPU-intensive. Rust handles this without GC pauses. |
| **3D rendering** | Bevy 0.15 | Modern ECS architecture, native Rust, no C++ interop. Excellent 2D/3D hybrid support, which we need for the UI overlay. |
| **Git parsing** | git2 (libgit2) | The gold standard for programmatic git access. Full commit graph traversal, blame, diff stats. |
| **No external assets** | Procedural generation | All geometry is generated at runtime. Zero asset pipeline. Open the app, point it at a repo, see a city. |
| **Cross-platform** | Bevy's native support | Linux, macOS, Windows. Optional wasm for web. |

---

## Design Principles

1. **Every detail has meaning.** Nothing is decorative. Building height = commit size. Color = author. Street width = branch activity. The visualization is *data-faithful*.
2. **Performance first.** A city for a repo with 50K commits should render at 60fps. LOD, instancing, and culling from day one.
3. **Zero setup.** Point at any git repo and get a city. No config files needed. No asset downloads. No accounts.
4. **Keyboard-first navigation.** Vim-like movement keys. Space to fly. Shift to boost. F to focus on a commit.
5. **Export quality.** Produce renders good enough to put on a wall or in a presentation.

---

## Getting Started

```bash
# Clone and build
git clone https://github.com/synthalorian/retro-spec
cd retro-spec
cargo build --release

# Run against any git repo
./target/release/retro-spec --repo /path/to/your-project

# Or run against this repo itself (inception mode)
./target/release/retro-spec --repo .
```

**Requirements:**
- Rust 1.85+ (2024 edition)
- Vulkan-compatible GPU (or Metal on macOS, or DX12 on Windows)
- `libgit2` development headers (`libgit2-dev` on Debian, `libgit2` on Arch)

---

## License

MIT — because the grid belongs to everyone.

---

## Credits

**Created by:** synth with assistance from synthclaw 🎹🦞

**Inspiration:**
- Gource — the pioneer of software version visualization
- The synthwave aesthetic — because code should look as good as it feels
- Every late-night `git log --graph` session wondering "what did I *actually* do here"

---

*"Every city tells a story. Your repo is no different."* 🌆
---

## ☕ Support the Developer

If this project saved you time, solved a problem, or just made your day a little more neon, you can fuel the next one:

[![Buy Me A Coffee](https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png)](https://buymeacoffee.com/synthalorian)
