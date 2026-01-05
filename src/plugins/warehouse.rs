use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::constants::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};
use crate::core::{WarehouseGrid, WarehouseZones};
use crate::plugins::navigation::NavigationPlugin;
use crate::systems::ui::{supervisor_panel, UiState};
use crate::systems::visualization::{draw_robot_paths, robot_color_system};

pub struct WarehousePlugins;

impl Plugin for WarehousePlugins {
    fn build(&self, app: &mut App) {
        // Créer zones d'abord
        let zones = WarehouseZones::default();

        // Créer grille avec racks comme obstacles
        let mut grid = WarehouseGrid::default();
        grid.apply_racks(&zones.racks);

        app.add_plugins(EguiPlugin::default())
            .insert_resource(zones)
            .insert_resource(grid)
            .init_resource::<UiState>()
            .insert_resource(ClearColor(Color::srgb(0.92, 0.92, 0.92)))
            .add_plugins(NavigationPlugin)
            .add_systems(Startup, (setup_camera, setup_scene, spawn_racks))
            .add_systems(EguiPrimaryContextPass, supervisor_panel)
            .add_systems(Update, (
                draw_grid,
                draw_zones,
                draw_robot_paths,
                robot_color_system,
                camera_controls,
            ));
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
        Transform::from_xyz(grid_center.x, 80.0, grid_center.z + 60.0)
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
        brightness: 800.0,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 2000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let floor_size = Vec2::new(
        GRID_WIDTH as f32 * CELL_SIZE,
        GRID_HEIGHT as f32 * CELL_SIZE,
    );

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, floor_size * 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.88, 0.88, 0.88),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(floor_size.x * 0.5, 0.0, floor_size.y * 0.5),
    ));
}

fn spawn_racks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    zones: Res<WarehouseZones>,
) {
    let rack_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.35, 0.2),
        perceptual_roughness: 0.8,
        ..default()
    });

    let height = 1.5;

    for rack in &zones.racks {
        let width = (rack.end.x - rack.start.x + 1) as f32 * CELL_SIZE;
        let length = (rack.end.y - rack.start.y + 1) as f32 * CELL_SIZE;

        let center_x = rack.start.x as f32 * CELL_SIZE + width * 0.5;
        let center_z = rack.start.y as f32 * CELL_SIZE + length * 0.5;

        let mesh = meshes.add(Cuboid::new(width, height, length));

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(rack_material.clone()),
            Transform::from_xyz(center_x, height * 0.5, center_z),
        ));
    }
}

fn draw_grid(mut gizmos: Gizmos) {
    let color = Color::srgba(0.7, 0.7, 0.7, 0.2);
    let w = GRID_WIDTH as f32 * CELL_SIZE;
    let h = GRID_HEIGHT as f32 * CELL_SIZE;
    let y = 0.01;

    for i in 0..=GRID_WIDTH {
        let x = i as f32 * CELL_SIZE;
        gizmos.line(Vec3::new(x, y, 0.0), Vec3::new(x, y, h), color);
    }

    for i in 0..=GRID_HEIGHT {
        let z = i as f32 * CELL_SIZE;
        gizmos.line(Vec3::new(0.0, y, z), Vec3::new(w, y, z), color);
    }
}

fn draw_zones(mut gizmos: Gizmos, zones: Res<WarehouseZones>) {
    let y = 0.02;

    for &pos in &zones.spawn_points {
        let x = pos.x as f32 * CELL_SIZE + CELL_SIZE * 0.5;
        let z = pos.y as f32 * CELL_SIZE + CELL_SIZE * 0.5;
        gizmos.rect(
            Isometry3d::new(Vec3::new(x, y, z), Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            Vec2::splat(CELL_SIZE * 0.7),
            Color::srgba(0.3, 0.5, 0.9, 0.3),
        );
    }

    for &pos in &zones.storage_cells {
        let x = pos.x as f32 * CELL_SIZE + CELL_SIZE * 0.5;
        let z = pos.y as f32 * CELL_SIZE + CELL_SIZE * 0.5;
        gizmos.rect(
            Isometry3d::new(Vec3::new(x, y, z), Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            Vec2::splat(CELL_SIZE * 0.5),
            Color::srgba(0.2, 0.75, 0.3, 0.4),
        );
    }

    for &pos in &zones.cargo_cells {
        let x = pos.x as f32 * CELL_SIZE + CELL_SIZE * 0.5;
        let z = pos.y as f32 * CELL_SIZE + CELL_SIZE * 0.5;
        gizmos.rect(
            Isometry3d::new(Vec3::new(x, y, z), Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            Vec2::splat(CELL_SIZE * 0.7),
            Color::srgba(0.95, 0.45, 0.2, 0.4),
        );
    }
}

fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = camera.single_mut() else { return };

    let speed = 30.0 * time.delta_secs();
    let forward = transform.forward().with_y(0.0).normalize_or_zero();
    let right = transform.right().with_y(0.0).normalize_or_zero();

    if keyboard.pressed(KeyCode::KeyW) { transform.translation += forward * speed; }
    if keyboard.pressed(KeyCode::KeyS) { transform.translation -= forward * speed; }
    if keyboard.pressed(KeyCode::KeyA) { transform.translation -= right * speed; }
    if keyboard.pressed(KeyCode::KeyD) { transform.translation += right * speed; }
    if keyboard.pressed(KeyCode::KeyQ) { transform.translation.y += speed; }
    if keyboard.pressed(KeyCode::KeyE) { transform.translation.y -= speed; }
}