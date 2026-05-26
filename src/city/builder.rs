use crate::city::planner::CityPlan;

/// Mesh data for a single building
pub struct BuildingMesh {
    pub position: (f32, f32, f32),
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub color: [f32; 4],
    pub commit_id: String,
    pub is_tagged: bool,
    pub timestamp: i64,
}

/// Mesh data for a street segment
pub struct StreetMesh {
    pub start: (f32, f32, f32),
    pub end: (f32, f32, f32),
    pub width: f32,
    pub color: [f32; 4],
    pub name: String,
}

/// Mesh data for a district ground tint
pub struct DistrictMesh {
    pub center: (f32, f32),
    pub size: (f32, f32),
    pub color: [f32; 4],
    pub name: String,
}

/// Mesh data for a merge intersection plaza — glowing ring
pub struct MergePlazaMesh {
    pub position: (f32, f32),
    pub radius: f32,
    pub color: [f32; 4],
    pub commit_id: String,
}

/// Mesh data for a skybridge — glass connector between branch lots
pub struct SkybridgeMesh {
    pub start: (f32, f32, f32),
    pub end: (f32, f32, f32),
    pub height: f32,
    pub color: [f32; 4],
}

/// Convert a CityPlan into renderable meshes
pub struct CityMeshes {
    pub buildings: Vec<BuildingMesh>,
    pub streets: Vec<StreetMesh>,
    pub districts: Vec<DistrictMesh>,
    pub plazas: Vec<MergePlazaMesh>,
    pub skybridges: Vec<SkybridgeMesh>,
}

pub fn build_city(plan: &CityPlan) -> CityMeshes {
    let buildings: Vec<BuildingMesh> = plan
        .lots
        .iter()
        .map(|lot| BuildingMesh {
            position: (lot.position.0, 0.0, lot.position.1),
            width: lot.width,
            height: lot.height,
            depth: lot.depth,
            color: lot.color,
            commit_id: lot.commit_id.clone(),
            is_tagged: lot.is_tagged,
            timestamp: lot.timestamp,
        })
        .collect();

    let streets: Vec<StreetMesh> = plan
        .streets
        .iter()
        .map(|s| StreetMesh {
            start: (s.start.0, 0.0, s.start.1),
            end: (s.end.0, 0.0, s.end.1),
            width: s.width,
            color: s.color,
            name: s.name.clone(),
        })
        .collect();

    let districts: Vec<DistrictMesh> = plan
        .districts
        .iter()
        .map(|d| DistrictMesh {
            center: ((d.bounds.0 + d.bounds.2) / 2.0, (d.bounds.1 + d.bounds.3) / 2.0),
            size: ((d.bounds.2 - d.bounds.0).abs(), (d.bounds.3 - d.bounds.1).abs()),
            color: d.color,
            name: d.name.clone(),
        })
        .collect();

    let plazas: Vec<MergePlazaMesh> = plan
        .plazas
        .iter()
        .map(|p| MergePlazaMesh {
            position: p.position,
            radius: p.radius,
            color: p.color,
            commit_id: p.commit_id.clone(),
        })
        .collect();

    let skybridges: Vec<SkybridgeMesh> = plan
        .skybridges
        .iter()
        .map(|s| SkybridgeMesh {
            start: (s.start.0, s.height, s.start.1),
            end: (s.end.0, s.height, s.end.1),
            height: s.height,
            color: s.color,
        })
        .collect();

    CityMeshes {
        buildings,
        streets,
        districts,
        plazas,
        skybridges,
    }
}