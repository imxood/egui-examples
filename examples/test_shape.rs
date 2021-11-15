use eframe::{
    egui::{
        self, emath::RectTransform, pos2, vec2, Color32, Frame, Painter, Pos2, Rect, Shape, Ui,
    },
    epi,
};

#[derive(Default)]
pub struct MyApp {}

impl MyApp {
    pub fn ui(&mut self, ui: &mut Ui) {
        self.paint(ui);
        // Make sure we allocate what we used (everything)
        // ui.expand_to_include_rect(painter.clip_rect());
    }

    fn paint(&mut self, ui: &mut Ui) {
        ui.ctx().request_repaint();

        let rect = ui.available_rect_before_wrap();

        let painter = Painter::new(ui.ctx().clone(), ui.layer_id(), rect);

        let mut shapes: Vec<Shape> = Vec::new();

        let shape = Shape::rect_filled(rect, 10.0, Color32::RED);
        shapes.push(shape);

        const ROW: i32 = 10;
        const COL: i32 = 10;
        const NODE_SIZE: f32 = 50.0;
        const INTERVAL: f32 = 10.0;

        let mut x_start = (rect.width() - (NODE_SIZE + INTERVAL) * ROW as f32 + INTERVAL) / 2.0;
        let mut y_start = (rect.height() - (NODE_SIZE + INTERVAL) * COL as f32 + INTERVAL) / 2.0;
        
        if x_start < 0.0 || y_start < 0.0 {
            return;
        }

        println!("rect: {:?}, w: {}, h: {}, x_start: {}, y_start: {}", &rect, rect.width(), rect.height(), x_start, y_start);

        x_start = rect.left() + x_start;
        y_start = rect.top() + y_start;

        for row in 0..ROW {
            let y_start = y_start + (NODE_SIZE + INTERVAL) * row as f32;
            for col in 0..COL {
                let shape = Shape::rect_filled(
                    Rect::from_two_pos(
                        pos2(
                            x_start + (NODE_SIZE + INTERVAL) * col as f32,
                            y_start,
                        ),
                        pos2(
                            x_start + (NODE_SIZE + INTERVAL) * col as f32 + NODE_SIZE,
                            y_start + NODE_SIZE,
                        ),
                    ),
                    10.0,
                    Color32::GOLD,
                );
                shapes.push(shape);
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
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(Box::new(MyApp::default()), native_options);
}
