use eframe::{
    egui::{self, vec2, Align2, FontData, FontDefinitions, FontFamily},
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

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "你好呀!"
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "DroidSansFallbackFull".to_owned(),
            FontData::from_static(include_bytes!("../fonts/DroidSansFallbackFull.ttf")),
        ); // .ttf and .otf supported

        fonts.font_data.insert(
            "UKIJCJK".to_owned(),
            FontData::from_static(include_bytes!("../fonts/UKIJCJK.ttf")),
        );

        let main_fonts = fonts.families.get_mut(&FontFamily::Proportional).unwrap();

        main_fonts.insert(0, "DroidSansFallbackFull".to_owned());
        main_fonts.insert(1, "UKIJCJK".to_owned());
        ctx.set_fonts(fonts);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
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
    eframe::run_native(Box::new(MyApp::default()), native_options);
}
