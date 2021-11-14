use eframe::{
    egui::{self, Color32, Frame, Painter, Pos2, Rect, Shape, Ui},
    epi,
};

#[derive(Default)]
pub struct MyApp {}

impl MyApp {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.ctx().request_repaint();
        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );
        self.paint(&painter);
        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());
    }

    fn paint(&mut self, painter: &Painter) {
        let mut shapes: Vec<Shape> = Vec::new();
        let rect = Shape::rect_filled(
            Rect::from_two_pos(Pos2::new(100.0, 100.0), Pos2::new(200.0, 200.0)),
            10.0,
            Color32::GOLD,
        );
        shapes.push(rect);

        let rect = Shape::rect_filled(
            Rect::from_two_pos(Pos2::new(250.0, 250.0), Pos2::new(350.0, 350.0)),
            10.0,
            Color32::GOLD,
        );
        shapes.push(rect);
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
