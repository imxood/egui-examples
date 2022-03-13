use bevy::{
    app::AppExit,
    prelude::*,
    window::{WindowId, WindowMode},
    winit::WinitWindows,
};
use bevy_egui::egui::{self, Align2, Context, Direction, Layout, ScrollArea, Sense, Window};

use crate::defines::APP_NAME;

use super::ui_state::UiState;

pub struct Titlebar {
    pub style_ui_open: bool,
}

impl Default for Titlebar {
    fn default() -> Self {
        Self {
            style_ui_open: Default::default(),
        }
    }
}

impl Titlebar {
    pub fn trigger_style_ui(&mut self) {
        self.style_ui_open = !self.style_ui_open;
    }

    pub fn style_ui(&mut self, ctx: &Context) {
        Window::new("style_ui")
            .collapsible(false)
            .open(&mut self.style_ui_open)
            .anchor(Align2::CENTER_CENTER, [0.0, -30.0])
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ctx.style_ui(ui);
                });
            });
    }

    pub fn show(
        ctx: &Context,
        ui: &mut egui::Ui,
        mut exit: EventWriter<AppExit>,
        mut windows: ResMut<Windows>,
        ui_state: &mut ResMut<UiState>,
        winit_windows: Res<WinitWindows>,
    ) {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.menu_button("选项", |ui| {
                    if ui.button("样式").clicked() {
                        ui_state.titlebar.trigger_style_ui();
                    }
                });
            });
            ui.with_layout(Layout::right_to_left(), |ui| {
                // 关闭窗口
                if ui.button("X").clicked() {
                    exit.send(AppExit);
                }
                // 最大化
                if ui.button("⛶").clicked() {
                    if let Some(window) = windows.get_primary_mut() {
                        let window_mode = window.mode();
                        if window_mode == WindowMode::Fullscreen {
                            ui_state.window_mode = WindowMode::Windowed;
                        } else {
                            ui_state.window_mode = WindowMode::Fullscreen;
                        }
                        window.set_mode(ui_state.window_mode);
                    }
                }
                // 最小化
                if ui.button("➖").clicked() {
                    if let Some(window) = windows.get_primary_mut() {
                        window.set_minimized(true);
                    }
                }
                // 设置
                if ui.button("⛭").clicked() {
                    ui_state.setting_window.trigger_show();
                }

                // 标题
                let (title_rect, res) =
                    ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());

                ui.allocate_ui_at_rect(title_rect, |ui| {
                    ui.with_layout(
                        Layout::centered_and_justified(Direction::LeftToRight),
                        |ui| {
                            ui.label(APP_NAME);
                        },
                    );
                });

                if res.double_clicked() {
                    if let Some(window) = winit_windows.get_window(WindowId::primary()) {
                        ui_state.maximized = !ui_state.maximized;
                        window.set_maximized(ui_state.maximized);
                    }
                } else if res.dragged() {
                    // 当拖动时, 如果不判断drag_delta, 直接进行 drag_window, 会导致 double_clicked 无法触发
                    let delta = res.drag_delta();
                    if delta.x != 0.0 && delta.y != 0.0 {
                        if let Some(window) = winit_windows.get_window(WindowId::primary()) {
                            window.drag_window().ok();
                        }
                    }
                }
            });
        });

        ui_state.titlebar.style_ui(ctx);
    }
}
