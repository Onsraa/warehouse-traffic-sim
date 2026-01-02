use bevy::prelude::*;
use warehouse_sim::plugins::warehouse::WarehousePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Warehouse Simulator".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WarehousePlugins)
        .run();
}
