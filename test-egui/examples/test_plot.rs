#![windows_subsystem = "windows"]

use eframe::{
<<<<<<< HEAD:examples/test_plot.rs
    egui::{self, Ui},
    epi, Frame,
=======
    egui::{
        self, Frame, Ui,
    },
    epi,
>>>>>>> 612b68aad580ffabb8fc826ba0316bc09b2a11eb:test-egui/examples/test_plot.rs
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
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            ui.label("First row, first column");
            ui.label("First row, second column");
            ui.end_row();

            ui.label("Second row, first column");
            ui.label("Second row, second column");
            ui.label("Second row, third column");
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Same");
                ui.label("cell");
            });
            ui.label("Third row, second column");
            ui.end_row();
        });
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
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunMode {
    Reactive,
    Continuous,
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Reactive
    }
}
