use bevy::{
    app::AppExit,
    prelude::*,
    window::{WindowId, WindowMode},
    winit::WinitWindows,
};
use bevy_egui::{
    egui::{
        self, vec2, Align, CollapsingHeader, Color32, Direction, FontData, FontDefinitions,
        FontFamily, Label, Layout, ScrollArea, Sense, SidePanel, TopBottomPanel, Ui, Visuals,
        Widget,
    },
    EguiContext, EguiPlugin, EguiSettings,
};

fn main() {
    App::new()
        .init_resource::<UiState>()
        .insert_resource(WindowDescriptor {
            decorations: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(init_egui)
        .add_system(ui)
        .run();
}

struct UiState {
    video_list: Vec<String>,
    maximized: bool,
    window_mode: WindowMode,
    scale_factor: f64,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            video_list: vec![
                "文件1".into(),
                "文件2".into(),
                "文件3".into(),
                "文件4".into(),
            ],
            maximized: true,
            window_mode: WindowMode::Windowed,
            scale_factor: 1.25,
        }
    }
}

fn init_egui(
    mut egui_ctx: ResMut<EguiContext>,
    mut egui_settings: ResMut<EguiSettings>,
    mut windows: ResMut<Windows>,
    ui_state: Res<UiState>,
) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_maximized(ui_state.maximized);
        window.set_mode(ui_state.window_mode);
    }

    egui_settings.scale_factor = ui_state.scale_factor;

    let ctx = egui_ctx.ctx_mut();

    let mut fonts = FontDefinitions::default();
    // 加载中文字体
    fonts.font_data.insert(
        "DroidSansFallbackFull".to_owned(),
        FontData::from_static(include_bytes!(
            "../../resources/fonts/DroidSansFallbackFull.ttf"
        )),
    );
    if let Some(font) = fonts
        .fonts_for_family
        // .families
        .get_mut(&FontFamily::Proportional)
    {
        font.insert(0, "DroidSansFallbackFull".to_owned());
    }

    ctx.set_fonts(fonts);

    ctx.set_visuals(Visuals::dark());
}

fn titlebar(
    ui: &mut Ui,
    mut exit: EventWriter<AppExit>,
    mut windows: ResMut<Windows>,
    ui_state: &mut ResMut<UiState>,
    winit_windows: Res<WinitWindows>,
) {
    ui.horizontal(|ui| {
        ui.with_layout(Layout::left_to_right(), |ui| {
            ui.menu_button("文件", |ui| {
                let _ = ui.button("添加视频文件");
                let _ = ui.button("Item1");
                let _ = ui.button("Item2");
                let _ = ui.button("Item3");
                let _ = ui.button("Item4");
            });
        });
        ui.with_layout(Layout::right_to_left(), |ui| {
            // 关闭窗口
            if ui.button("x").clicked() {
                exit.send(AppExit);
            }
            // 最大化
            if ui.button("口").clicked() {
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
            if ui.button("-").clicked() {
                if let Some(window) = windows.get_primary_mut() {
                    window.set_minimized(true);
                }
            }
            // 标题
            let (title_rect, res) =
                ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());

            ui.allocate_ui_at_rect(title_rect, |ui| {
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        ui.label("RS-FFMPEG-PLAYER 播放器");
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
}

fn ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    exit: EventWriter<AppExit>,
    windows: ResMut<Windows>,
    winit_windows: Res<WinitWindows>,
) {
    let ctx = egui_ctx.ctx_mut();

    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        titlebar(ui, exit, windows, &mut ui_state, winit_windows)
    });

    let video_list = &mut ui_state.video_list;

    SidePanel::right("right_side_panel")
        .min_width(250.0)
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| if ui.button("添加").clicked() {});

                CollapsingHeader::new("播放列表")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.with_layout(
                            Layout::top_down(Align::Min).with_cross_justify(true),
                            |ui| {
                                for (i, video) in video_list.clone().iter().enumerate() {
                                    Label::new(video).sense(Sense::click()).ui(ui).context_menu(
                                        |ui| {
                                            if ui.button("移除").clicked() {
                                                video_list.remove(i);
                                                println!("移除 {}", i);
                                            }
                                        },
                                    );
                                }
                            },
                        );
                    });
            });
        });

    egui::Window::new("basic_window")
        .default_height(500.0)
        .show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();
            // 按钮背景色
            style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(25, 66, 124);
            // 窗口背景色
            style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(3, 45, 100);
            // 窗口边框颜色
            style.visuals.widgets.noninteractive.bg_stroke.color =
                Color32::from_rgba_premultiplied(46, 46, 46, 0);
            // 字体颜色
            style.visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
            // 鼠标经过时 按钮背景颜色
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(4, 148, 210);
            // 按钮被激活时, 背景颜色
            style.visuals.widgets.active.bg_fill = Color32::from_rgb(37, 95, 226);

            // 设置间距
            style.spacing.button_padding = vec2(16.0, 4.0);

            style.ui(ui);

            ctx.set_style(style);
        });
}
