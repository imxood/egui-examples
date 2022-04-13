use eframe::{
    egui::{self, vec2, Align2, FontData, FontDefinitions, FontFamily, Visuals},
    epi,
};

pub struct MyApp {
    label: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "你好".to_owned(),
        }
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext) -> Box<Self> {
        let app = Self::default();
        let ctx = &cc.egui_ctx;

        // 设置主题
        ctx.set_visuals(Visuals::light());
        ctx.set_pixels_per_point(10.0);

        // 调试
        ctx.set_debug_on_hover(true);

        // 设置字体
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "DroidSansFallbackFull".to_owned(),
            FontData::from_static(include_bytes!("../../misc/fonts/DroidSansFallbackFull.ttf")),
        ); // .ttf and .otf supported
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "DroidSansFallbackFull".to_owned());
        ctx.set_fonts(fonts);
        Box::new(app)
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        let Self { label } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
            ui.heading("Center Panel");
            ui.vertical_centered(|ui| {
                ui.label(format!("世界， {}", label));
            });

            ui.horizontal(|ui| {
                ui.label("title:");
                ui.text_edit_singleline(label);
            });

            egui::Window::new("测试窗口")
                .id(egui::Id::new("test_window")) // required since we change the title
                .default_size(vec2(100.0, 60.0))
                .min_width(100.0)
                .min_height(50.0)
                .resizable(false)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, -50.0])
                .show(ctx, |_ui| {});
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(200.0, 150.0)),
        ..Default::default()
    };
    eframe::run_native("hello", native_options, Box::new(|cc| MyApp::new(cc)));
}
