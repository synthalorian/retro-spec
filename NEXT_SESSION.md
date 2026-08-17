# RetroSpec — Next Session Plan

## What We Just Built (This Session)

All three Phase 2 leftovers landed. **Phase 2 — City Builder (v0.2.0) is now complete.** 🎉

### ✅ Merge Intersection Plazas
- **Detects merge commits** (`c.is_merge`) and finds cross-branch parent relationships
- **Spawns glowing gold torus rings** at the Z-midpoint between intersecting branch streets
- **Inner translucent disc** fills the ring for a glowing plaza effect
- **Color:** Gold (#FFB300) with unlit emissive
- **Files touched:** `planner.rs` (build_merge_plazas), `builder.rs` (MergePlazaMesh), `streets.rs` (spawn_merge_plazas), `main.rs` (wired into setup_scene)

### ✅ Author & Directory Legend Overlay
- **Top-right corner panel** showing color-coded author list with text swatches
- **Directory/district section** below authors with muted colors
- **Limit 8 directories** shown to avoid clutter, "+N more…" overflow indicator
- **Press L to toggle visibility** on/off
- **Data auto-populated** from commit authors and city plan districts
- **Files touched:** `legend.rs` (full rewrite with Bevy Resource + UI), `mod.rs`, `main.rs`

### ✅ Grid Optimization
- **~200 individual line entities → 1 single mesh** with custom vertex data
- **101 X-axis + 101 Z-axis lines** all batched into one TriangleList mesh
- **~2000× fewer draw calls** for the grid layer
- **Same visual result** — same grid color, same unlit material
- **Files touched:** `terrain.rs` (full rewrite)

---

## What's Next (Phase 3 — Polish & Performance, v0.3.0)

### LOD System — Level of Detail
Buildings far from camera render as simple cuboids with flat color. Close ones keep emissive glow. Use Bevy's `LevelOfDetail` or a distance-based material swap in Update.

**Key files:** `render/buildings.rs` + new `render/lod.rs`

### Window Grid Textures
Procedural building facades with lit/unlit windows. Generate a texture procedurally using `Image` data, apply as albedo texture. Some windows lit (random), some dark.

**Key files:** `render/buildings.rs` + new utility

### Particle System
Ambient "digital rain", sparks near buildings, neon haze. Use Bevy's built-in particle system (Bevy 0.15 has `bevy_particle_systems` or built-in). Falling digital rain lines near tall buildings.

**Key files:** `render/particles.rs` (currently a stub)

### Cherry-Pick Skybridges
Connecting buildings across branches that share cherry-picked commits. Detect same commit content across branches, draw glass tube bridges between them.

**Key files:** `city/planner.rs` → `render/streets.rs`

### Tag Landmarks — Rotating Beacons
Tagged commits currently have wider base + gold tint. Give them rotating beacon lights on top (point light + animated cone mesh).

**Key files:** `render/buildings.rs`

---

## Verification

```bash
# Quick compile check
cargo check

# Visual test with open_habit
cargo run -- -r /home/synthalorian 🎹🤺/projects/open_habit

# Stats sanity
cargo run -- -r /home/synthalorian 🎹🤺/projects/retro-spec --stats
```

**Test repos:**
- `/home/synthalorian 🎹🤺/projects/retro-spec` — simple, has merge commits if you've PR'd
- `/home/synthalorian 🎹🤺/projects/open_habit` — 9 commits, 3 tags, single branch
- `/home/synthalorian 🎹🤺/projects/hermes-agent` — 800+ commits, 10+ branches, merges galore

---

*"A repository is a city built over time. Phase 2 complete — the city has districts, boulevards, crossroads, and a timeline. Now we polish."* 🏙️🎹🦈