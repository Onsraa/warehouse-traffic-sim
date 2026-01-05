use bevy::prelude::*;

use crate::components::{GridPosition, Loaded, PlannedPath, Robot};
use crate::core::WarehouseGrid;

/// Met à jour la couleur des robots selon leur état de chargement
pub fn robot_color_system(
    robots: Query<(&Loaded, &MeshMaterial3d<StandardMaterial>), (With<Robot>, Changed<Loaded>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (loaded, material_handle) in &robots {
        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = if loaded.0 {
                Color::srgb(0.8, 0.3, 0.1) // Orange = chargé
            } else {
                Color::srgb(0.2, 0.6, 0.2) // Vert = non chargé
            };
        }
    }
}

/// Dessine les chemins planifiés des robots
pub fn draw_robot_paths(
    mut gizmos: Gizmos,
    robots: Query<(&PlannedPath, &Loaded), With<Robot>>,
    grid: Res<WarehouseGrid>,
) {
    for (path, loaded) in &robots {
        let waypoints = path.remaining();
        if waypoints.len() < 2 {
            continue;
        }

        let color = if loaded.0 {
            Color::srgba(0.9, 0.4, 0.1, 0.6) // Orange transparent
        } else {
            Color::srgba(0.2, 0.7, 0.2, 0.6) // Vert transparent
        };

        for window in waypoints.windows(2) {
            let (from_pos, _) = window[0];
            let (to_pos, _) = window[1];

            let (fx, fz) = grid.grid_to_world(from_pos);
            let (tx, tz) = grid.grid_to_world(to_pos);

            gizmos.line(Vec3::new(fx, 0.3, fz), Vec3::new(tx, 0.3, tz), color);
        }
    }
}
