use crate::constants::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};
use crate::core::WarehouseGrid;
use bevy::prelude::*;

pub struct WarehousePlugins;

impl Plugin for WarehousePlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<WarehouseGrid>();
        app.add_systems(Startup, (setup_camera, setup_scene));
    }
}

fn setup_camera(mut commands: Commands) {
    let grid_center = Vec3::new(
        GRID_WIDTH as f32 * CELL_SIZE * 0.5,
        0.0,
        GRID_HEIGHT as f32 * CELL_SIZE * 0.5,
    );

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(grid_center.x, 90.0, grid_center.z + 30.0)
            .looking_at(grid_center, Vec3::Y),
    ));
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
        affects_lightmapped_meshes: false,
    });

    // Sol gris très clair
    let floor_size = Vec2::new(
        GRID_WIDTH as f32 * CELL_SIZE,
        GRID_HEIGHT as f32 * CELL_SIZE,
    );

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, floor_size * 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.95, 0.95),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(floor_size.x * 0.5, 0.0, floor_size.y * 0.5),
    ));
}