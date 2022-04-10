#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui::{
        self, emath::RectTransform, epaint::Mesh, pos2, vec2, Color32, Pos2, Rect, Sense, Shape, Ui,
    },
    epi, Frame,
};
use test_egui::frame_history::FrameHistory;

fn main() {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "hello",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

#[derive(Default)]
pub struct MyApp {
    frame_history: FrameHistory,
    run_mode: RunMode,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl MyApp {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.ctx().request_repaint();
        let (res, painter) =
            ui.allocate_painter(ui.available_rect_before_wrap().size(), Sense::click());
        if res.hovered() {
            println!("painter hovered");
        }
        if res.clicked() {
            println!("painter clicked");
        }

        const ROW: i32 = 10 * 4;
        const COL: i32 = 12 * 4;
        const NODE_SIZE: f32 = 20.0;
        const INTERVAL: f32 = 5.0;

        let rect = painter.clip_rect();

        // 矩形空间, 每一行 和 每一列 的开头和结尾 都填充一个 NODE_SIZE
        let width = (NODE_SIZE + INTERVAL) * (COL + 2) as f32 - INTERVAL;
        let height = (NODE_SIZE + INTERVAL) * (ROW + 2) as f32 - INTERVAL;

        // 从给出的一个矩形空间 转换到 屏幕尺寸 空间, 并缩放
        let to_screen =
            RectTransform::from_to(Rect::from_min_size(Pos2::ZERO, vec2(width, height)), rect);

        let mut mesh = Mesh::default();

        // 每一行 和 每一列 的开头和结尾 都填充一个 NODE_SIZE
        for row in 0..ROW {
            let y_start = (NODE_SIZE + INTERVAL) * (row + 1) as f32;
            for col in 0..COL {
                let x_start = (NODE_SIZE + INTERVAL) * (col + 1) as f32;
                mesh.add_colored_rect(
                    Rect::from_two_pos(
                        to_screen * pos2(x_start, y_start),
                        to_screen * pos2(x_start + NODE_SIZE, y_start + NODE_SIZE),
                    ),
                    Color32::GOLD,
                );
            }
        }
        painter.add(Shape::Mesh(mesh));
    }

    fn run_mode_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let run_mode = &mut self.run_mode;
            ui.label("Mode:");
            ui.radio_value(run_mode, RunMode::Reactive, "Reactive")
                .on_hover_text("Repaint when there are animations or input (e.g. mouse movement)");
            ui.radio_value(run_mode, RunMode::Continuous, "Continuous")
                .on_hover_text("Repaint everything each frame");
        });

        if self.run_mode == RunMode::Continuous {
            ui.label(format!(
                "Repainting the UI each frame. FPS: {:.1}",
                self.frame_history.fps()
            ));
        } else {
            ui.label("Only running UI code when there are animations or input");
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        // frame history
        self.frame_history
            .on_new_frame(ctx.input().time, frame.info().cpu_usage);

        egui::SidePanel::right("right").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("💻 Test");
                ui.label("hello, the world");
            });

            // debug on hover
            let mut debug_on_hover = ui.ctx().debug_on_hover();
            ui.checkbox(&mut debug_on_hover, "🐛 Debug on hover")
                .on_hover_text("Show structure of the ui when you hover with the mouse");
            ui.ctx().set_debug_on_hover(debug_on_hover);

            self.run_mode_ui(ui);

            // fps
            self.frame_history.ui(ui);
        });

        egui::CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.ui(ui));

        let events = &ctx.output().events;
        if !events.is_empty() {
            println!("events: {:?}", events);
        }
    }
}

/// How often we repaint the demo app by default
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunMode {
    /// This is the default for the demo.
    ///
    /// If this is selected, egui is only updated if are input events
    /// (like mouse movements) or there are some animations in the GUI.
    ///
    /// Reactive mode saves CPU.
    ///
    /// The downside is that the UI can become out-of-date if something it is supposed to monitor changes.
    /// For instance, a GUI for a thermostat need to repaint each time the temperature changes.
    /// To ensure the UI is up to date you need to call `egui::Context::request_repaint()` each
    /// time such an event happens. You can also chose to call `request_repaint()` once every second
    /// or after every single frame - this is called `Continuous` mode,
    /// and for games and interactive tools that need repainting every frame anyway, this should be the default.
    Reactive,

    /// This will call `egui::Context::request_repaint()` at the end of each frame
    /// to request the backend to repaint as soon as possible.
    ///
    /// On most platforms this will mean that egui will run at the display refresh rate of e.g. 60 Hz.
    ///
    /// For this demo it is not any reason to do so except to
    /// demonstrate how quickly egui runs.
    ///
    /// For games or other interactive apps, this is probably what you want to do.
    /// It will guarantee that egui is always up-to-date.
    Continuous,
}

/// Default for demo is Reactive since
/// 1) We want to use minimal CPU
/// 2) There are no external events that could invalidate the UI
///    so there are no events to miss.
impl Default for RunMode {
    fn default() -> Self {
        RunMode::Reactive
    }
}
