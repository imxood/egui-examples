use eframe::{
    egui::{self, FontData, FontDefinitions, FontFamily},
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
        "你好呀！"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "DroidSansFallbackFull".to_owned(),
            FontData::from_static(include_bytes!("../src/fonts/DroidSansFallbackFull.ttf")),
        ); // .ttf and .otf supported
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "DroidSansFallbackFull".to_owned());
        ctx.set_fonts(fonts);
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
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
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), native_options);
}
