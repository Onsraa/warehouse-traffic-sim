use bevy::{prelude::*, text::FontSmoothing};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
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
        /*
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 42.0,
                    font: default(),
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: true,
                    min_fps: 30.0,
                    target_fps: 144.0,
                },
                text_color: Default::default(),
            },
        })
         */
        .run();
}
