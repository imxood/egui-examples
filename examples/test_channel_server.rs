use channel_server::{ChannelService, Route};
use eframe::{
    egui::{self, vec2, Align2, FontData, FontDefinitions, FontFamily},
    epi,
};

pub struct MyApp {
    label: String,
    dark_mode: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "你好".to_owned(),
            dark_mode: false,
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

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { label, dark_mode } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // let dark_mode = ctx.style().visuals.dark_mode;
            egui::menu::bar(ui, |ui| {
                //logo
                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    ui.heading("aNOOBis");
                    let mut dragbtn = egui::Button::new("dragme");
                    dragbtn = dragbtn.sense(egui::Sense::drag());
                    let btnresponse = ui.add(dragbtn);
                    if btnresponse.dragged() {
                        println!("draged: {:?}", btnresponse.drag_delta());
                        // TODO: Find a way to set window pos in here. Or even how to get the window handle
                        frame.drag_window()
                    }
                    egui::warn_if_debug_build(ui);
                });
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.small_button("❌").clicked() {
                        frame.quit();
                    }
                    if ui.small_button("−").clicked() {
                        //todo
                    }
                    if ui
                        .small_button({
                            if *dark_mode {
                                "🌞"
                            } else {
                                "🌙"
                            }
                        })
                        .clicked()
                    {
                        *dark_mode = !*dark_mode;
                    }
                });
            });
        });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     egui::warn_if_debug_build(ui);
        //     ui.heading("Center Panel");
        //     ui.vertical_centered(|ui| {
        //         ui.label(format!("世界， {}", label));
        //     });

        //     ui.horizontal(|ui| {
        //         ui.label("title:");
        //         ui.text_edit_singleline(label);
        //     });

        //     egui::Window::new("测试窗口")
        //         .id(egui::Id::new("test_window")) // required since we change the title
        //         .default_size(vec2(100.0, 60.0))
        //         .min_width(100.0)
        //         .min_height(50.0)
        //         .resizable(false)
        //         .collapsible(false)
        //         .anchor(Align2::CENTER_CENTER, [0.0, -50.0])
        //         .show(ctx, |_ui| {});
        // });
    }
}

struct AuthService {
    name: String,
    password: String,
}

impl AuthService {
    pub fn auth() {}
}

#[derive(Default)]
struct NumberService {
    /// 目标数
    num: i32,
    /// 向前, 递增
    forward: bool,
}

impl NumberService {
    pub fn new() {}
    pub fn reduce(&mut self) {
        self.forward = false;
    }

    pub fn plus(&mut self) {
        self.forward = true;
    }

    pub fn run(&mut self) {
        loop {
            if self.forward {
                self.num = self.num.wrapping_add(1);
            }
        }
    }
}

fn main() {
    // let ep = Route::new().at("/num/reduce", ep);

    // let (mut client, topic) = ChannelService::start(ep);

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(200.0, 150.0)),
        decorated: false,
        ..Default::default()
    };
    eframe::run_native(
        "hello",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}
