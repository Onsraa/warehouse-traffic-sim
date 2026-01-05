use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::{
    ActionTimer, Destination, GridPosition, Loaded, Mission, MissionPhase,
    PlannedPath, Robot, RobotState, State, Velocity,
};
use crate::constants::{DROPOFF_DURATION, PICKUP_DURATION};
use crate::core::SpaceTimeTable;
use crate::systems::spawner::SpawnQueue;

#[derive(Resource, Default)]
pub struct UiState {
    pub collapsed: bool,
    pub selected_robot: Option<Entity>,
}

pub fn supervisor_panel(
    mut contexts: EguiContexts,
    robots: Query<(
        Entity,
        &GridPosition,
        &Destination,
        &State,
        &Loaded,
        &Velocity,
        &Mission,
        &PlannedPath,
        Option<&ActionTimer>,
    ), With<Robot>>,
    space_time: Res<SpaceTimeTable>,
    spawn_queue: Res<SpawnQueue>,
    mut ui_state: ResMut<UiState>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    // Style sobre
    ctx.style_mut(|style| {
        style.visuals.window_shadow = egui::epaint::Shadow::NONE;
        style.visuals.window_fill = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 250);
        style.visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(220));
        style.spacing.item_spacing = egui::vec2(4.0, 4.0);
    });

    let screen = ctx.screen_rect();
    let panel_width = 280.0;
    let padding = 12.0;

    egui::Window::new("🏭 Superviseur")
        .fixed_pos(egui::pos2(screen.width() - panel_width - padding, padding))
        .default_width(panel_width)
        .collapsible(true)
        .movable(false)
        .resizable(false)
        .show(ctx, |ui| {
            // Stats compactes
            ui.horizontal(|ui| {
                let spawned = spawn_queue.spawned_count;
                let total = spawn_queue.total;
                let tick = space_time.current_tick();
                let loaded_count = robots.iter().filter(|r| r.4.0).count();

                compact_stat(ui, "🤖", format!("{}/{}", spawned, total), egui::Color32::from_rgb(59, 130, 246));
                compact_stat(ui, "📦", loaded_count.to_string(), egui::Color32::from_rgb(234, 88, 12));
                compact_stat(ui, "⏱", format!("{}", tick), egui::Color32::from_rgb(107, 114, 128));
            });

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(4.0);

            // Liste robots
            egui::ScrollArea::vertical()
                .max_height(320.0)
                .show(ui, |ui| {
                    let mut sorted: Vec<_> = robots.iter().collect();
                    sorted.sort_by_key(|r| r.0.index());

                    for (entity, pos, dest, state, loaded, vel, mission, path, timer) in sorted {
                        let is_selected = ui_state.selected_robot == Some(entity);

                        let frame = egui::Frame::none()
                            .fill(if is_selected {
                                egui::Color32::from_rgb(239, 246, 255)
                            } else {
                                egui::Color32::TRANSPARENT
                            })
                            .inner_margin(4.0)
                            .corner_radius(4.0)
                            .stroke(egui::Stroke::new(
                                0.5,
                                egui::Color32::from_gray(if is_selected { 180 } else { 235 }),
                            ));

                        frame.show(ui, |ui| {
                            let resp = ui.interact(
                                ui.max_rect(),
                                egui::Id::new(("robot", entity)),
                                egui::Sense::click(),
                            );
                            if resp.clicked() {
                                ui_state.selected_robot = if is_selected { None } else { Some(entity) };
                            }

                            ui.horizontal(|ui| {
                                // ID avec indicateur chargement
                                let (icon, color) = if loaded.0 {
                                    ("📦", egui::Color32::from_rgb(234, 88, 12))
                                } else {
                                    ("○", egui::Color32::from_rgb(34, 197, 94))
                                };

                                ui.label(egui::RichText::new(format!("{} #{}", icon, entity.index()))
                                    .size(11.0).strong().color(color));

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    state_badge(ui, state.0);
                                });
                            });

                            // Barre de progression si action en cours
                            if let Some(t) = timer {
                                let total_duration = match mission.phase {
                                    MissionPhase::PickingUp => PICKUP_DURATION,
                                    MissionPhase::DroppingOff => DROPOFF_DURATION,
                                    _ => 1.0,
                                };
                                let progress = t.progress(total_duration);

                                ui.add_space(2.0);
                                let bar_color = match mission.phase {
                                    MissionPhase::PickingUp => egui::Color32::from_rgb(234, 179, 8),
                                    MissionPhase::DroppingOff => egui::Color32::from_rgb(59, 130, 246),
                                    _ => egui::Color32::GRAY,
                                };

                                let (rect, _) = ui.allocate_exact_size(
                                    egui::vec2(ui.available_width(), 3.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter().rect_filled(rect, 2.0, egui::Color32::from_gray(230));
                                let mut fill_rect = rect;
                                fill_rect.set_width(rect.width() * progress);
                                ui.painter().rect_filled(fill_rect, 2.0, bar_color);
                            }

                            ui.add_space(2.0);

                            // Infos compactes
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 8.0;

                                ui.label(egui::RichText::new(format!("({},{})", pos.0.x, pos.0.y))
                                    .size(10.0).color(egui::Color32::from_gray(120)));

                                ui.label(egui::RichText::new(format!("→({},{})", dest.0.x, dest.0.y))
                                    .size(10.0).color(egui::Color32::from_gray(100)));

                                let remaining = path.remaining().len();
                                if remaining > 0 {
                                    ui.label(egui::RichText::new(format!("🛤{}", remaining))
                                        .size(10.0).color(egui::Color32::from_gray(140)));
                                }
                            });
                        });

                        ui.add_space(2.0);
                    }
                });
        });

    Ok(())
}

fn compact_stat(ui: &mut egui::Ui, icon: &str, value: String, color: egui::Color32) {
    egui::Frame::none()
        .fill(color.gamma_multiply(0.1))
        .inner_margin(egui::Margin::symmetric(6, 3))
        .corner_radius(3.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                ui.label(egui::RichText::new(icon).size(10.0));
                ui.label(egui::RichText::new(value).size(10.0).strong().color(color));
            });
        });
}

fn state_badge(ui: &mut egui::Ui, state: RobotState) {
    let (text, color) = match state {
        RobotState::Idle => ("IDLE", egui::Color32::from_rgb(156, 163, 175)),
        RobotState::Moving => ("MOV", egui::Color32::from_rgb(34, 197, 94)),
        RobotState::Loading => ("LOAD", egui::Color32::from_rgb(234, 179, 8)),
        RobotState::Unloading => ("DROP", egui::Color32::from_rgb(59, 130, 246)),
        RobotState::Charging => ("CHG", egui::Color32::from_rgb(168, 85, 247)),
        RobotState::Fault => ("ERR", egui::Color32::from_rgb(239, 68, 68)),
    };

    egui::Frame::none()
        .fill(color)
        .inner_margin(egui::Margin::symmetric(4, 1))
        .corner_radius(2.0)
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).size(9.0).color(egui::Color32::WHITE));
        });
}