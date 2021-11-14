use eframe::{
    egui::{self, Color32, Frame, Painter, Pos2, Rect, Shape, Ui},
    epi,
};

#[derive(Default)]
pub struct MyApp {}

impl MyApp {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.ctx().request_repaint();
        let paint_rect = ui.available_rect_before_wrap();
        let painter = Painter::new(ui.ctx().clone(), ui.layer_id(), paint_rect);
        self.paint(&painter);
        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());
    }

    fn paint(&mut self, painter: &Painter) {
        let mut shapes: Vec<Shape> = Vec::new();
        let x_start = 300.0;
        for row in 0..10 {
            for col in 0..10 {
                let rect = Shape::rect_filled(
                    Rect::from_two_pos(
                        Pos2::new(x_start + 50.0 * col as f32 + 10.0, 50.0 * row as f32 + 10.0),
                        Pos2::new(x_start + 50.0 * (col + 1) as f32, 50.0 * (row + 1) as f32),
                    ),
                    10.0,
                    Color32::GOLD,
                );
                shapes.push(rect);
            }
        }

        painter.extend(shapes);
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "你好呀！"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::SidePanel::left("left").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("💻 Test");
                ui.label("hello, the world");
            });
            let mut debug_on_hover = ui.ctx().debug_on_hover();
            ui.checkbox(&mut debug_on_hover, "🐛 Debug on hover")
                .on_hover_text("Show structure of the ui when you hover with the mouse");
            ui.ctx().set_debug_on_hover(debug_on_hover);
        });
        egui::CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.ui(ui));
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), native_options);
}
