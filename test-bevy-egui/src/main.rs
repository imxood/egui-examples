use bevy::{app::AppExit, prelude::*, window::WindowId, winit::WinitWindows};
use bevy_egui::{
    egui::{
        style::Margin, CentralPanel, Color32, Frame, Label, Sense, SidePanel,
        TopBottomPanel, Widget,
    },
    EguiContext, EguiPlugin, EguiSettings,
};
use defines::icons::ICON_LOGO;
use resources::fonts::load_fonts;
use ui::{titlebar_ui::Titlebar, ui_state::UiState};
use winit::window::Icon;

mod defines;
mod resources;
mod ui;
mod utils;

fn main() {
    App::new()
        .init_resource::<UiState>()
        .insert_resource(WindowDescriptor {
            decorations: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(icon_setup)
        .add_startup_system(egui_setup)
        .add_system(ui)
        .run();
}

/// 设置应用图标
fn icon_setup(windows: Res<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(ICON_LOGO)
            .expect("Failed to open logo icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}

/// egui环境初始化
fn egui_setup(
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

    ctx.set_fonts(load_fonts());

    ctx.set_style(ui_state.theme.blue_style_clone());
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
        Titlebar::show(ctx, ui, exit, windows, &mut ui_state, winit_windows)
    });

    SidePanel::right("right_side_panel")
        .min_width(200.0)
        .default_width(250.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 有click的response, 都可以使用 context_menu
                Label::new("右击我?")
                    .sense(Sense::click())
                    .ui(ui)
                    .context_menu(|ui| {
                        ui.menu_button("Are you ok?", |ui| if ui.button("I'm fine").clicked() {});
                        if ui.button("I'm fine").clicked() {}
                    });
            });
        });

    // 设置背景
    let frame = Frame {
        margin: Margin::symmetric(8.0, 8.0),
        fill: Color32::LIGHT_GRAY,
        ..Default::default()
    };

    CentralPanel::default().frame(frame).show(ctx, |ui| {
        ui.horizontal(|_ui| {});
    });

    ui_state.setting_window.show(ctx);
}
