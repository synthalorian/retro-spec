use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

/// Spawn the ground plane with neon grid overlay — single mesh for performance.
/// Replaces ~200 individual line entities with one batched draw call.
pub fn spawn_terrain(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    grid_color: Color,
    ground_color: Color,
) {
    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(200.0, 200.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: ground_color,
            ..Default::default()
        })),
        Transform::from_xyz(0.0, -0.1, 0.0),
    ));

    // ── Grid: single mesh from all line vertices ──
    let grid_size = 100.0;
    let step = 2.0;
    let line_width = 0.04;

    let half_width = line_width / 2.0;
    let num_steps = (grid_size * 2.0 / step) as u32 + 1;

    // Each line is a quad (4 vertices, 6 indices)
    // X-lines: num_steps lines along X, spanning Z
    // Z-lines: num_steps lines along Z, spanning X
    let total_lines = num_steps * 2;
    let vertex_count = (total_lines * 4) as usize;
    let index_count = (total_lines * 6) as usize;

    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);
    let mut indices: Vec<u32> = Vec::with_capacity(index_count);

    let mut vertex_idx: u32 = 0;

    // X-axis lines (along X at each Z step)
    for i in 0..num_steps {
        let z = -grid_size + i as f32 * step;

        // Thin quad spanning X at z ± half_width
        // v0-v1-v2-v3: rectangle (X from -grid_size to +grid_size, Z from z-hw to z+hw)
        let v0 = Vec3::new(-grid_size, 0.0, z - half_width);
        let v1 = Vec3::new(grid_size, 0.0, z - half_width);
        let v2 = Vec3::new(grid_size, 0.0, z + half_width);
        let v3 = Vec3::new(-grid_size, 0.0, z + half_width);

        positions.extend_from_slice(&[v0, v1, v2, v3]);
        // All normals point up
        normals.extend_from_slice(&[Vec3::Y; 4]);
        // Simple UVs (not used for unlit grid)
        uvs.extend_from_slice(&[
            [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        ]);

        // Two triangles: v0-v1-v2, v0-v2-v3
        indices.extend_from_slice(&[
            vertex_idx, vertex_idx + 1, vertex_idx + 2,
            vertex_idx, vertex_idx + 2, vertex_idx + 3,
        ]);
        vertex_idx += 4;
    }

    // Z-axis lines (along Z at each X step)
    for i in 0..num_steps {
        let x = -grid_size + i as f32 * step;

        // Thin quad spanning Z at x ± half_width
        let v0 = Vec3::new(x - half_width, 0.0, -grid_size);
        let v1 = Vec3::new(x + half_width, 0.0, -grid_size);
        let v2 = Vec3::new(x + half_width, 0.0, grid_size);
        let v3 = Vec3::new(x - half_width, 0.0, grid_size);

        positions.extend_from_slice(&[v0, v1, v2, v3]);
        normals.extend_from_slice(&[Vec3::Y; 4]);
        uvs.extend_from_slice(&[
            [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
        ]);

        indices.extend_from_slice(&[
            vertex_idx, vertex_idx + 1, vertex_idx + 2,
            vertex_idx, vertex_idx + 2, vertex_idx + 3,
        ]);
        vertex_idx += 4;
    }

    let mut grid_mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
    grid_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    grid_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    grid_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    grid_mesh.insert_indices(Indices::U32(indices));

    // Single entity for the entire grid
    commands.spawn((
        Mesh3d(meshes.add(grid_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: grid_color,
            unlit: true,
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}