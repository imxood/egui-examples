use eframe::{
    egui::{
        self, vec2, Align, Button, Color32, FontData, FontDefinitions, FontFamily, RichText, Sense,
        Stroke, TextEdit, TextStyle, Ui, Visuals, Widget, WidgetText,
    },
    epi,
};

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

pub struct MyApp {
    text_code: String,
    status: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text_code: "xx1839894303920901213".into(),
            status: "PASS".into(),
        }
    }
}

impl MyApp {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("hello world");
        });
        egui::Grid::new("some_unique_id")
            // 长度为2, 第2列宽度是剩余的宽度
            .num_columns(2)
            .show(ui, |ui| {
                /*
                    第 1 行
                */

                ui.with_layout(egui::Layout::bottom_up(Align::Center), |ui| {
                    // 发现 手动设置高度后, 就感觉是浮动了?
                    // println!("ui.available_size(): {:?}", ui.available_size());

                    // 代码实现仿按钮

                    // 创建文字
                    let text =
                        WidgetText::RichText(RichText::new("测试工位").color(Color32::LIGHT_RED));
                    let text = text.into_galley(ui, None, ui.available_width(), TextStyle::Button);

                    // 分配文字区域
                    // let size = text.size() + vec2(20.0, 0.0);
                    let size = vec2(200.0, 20.0);
                    // println!("size: {:?}", &size);
                    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

                    // 获取可视化
                    let visuals = ui.style().interact(&response);

                    // 画形状
                    ui.painter().rect(
                        rect,
                        3.0,
                        ui.ctx().style().visuals.window_fill(),
                        Stroke::new(1.0, Color32::RED),
                    );

                    // 画文字
                    let text_pos = ui.layout().align_size_within_rect(text.size(), rect).min;
                    text.paint_with_visuals(ui.painter(), text_pos, visuals);
                });

                ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
                    Button::new(RichText::new(self.status.clone()).color(Color32::GREEN))
                        .fill(Color32::YELLOW)
                        .stroke(Stroke::none())
                        .ui(ui);
                });

                ui.end_row();

                /*
                    第 2 行
                */

                ui.with_layout(egui::Layout::bottom_up(Align::Center), |ui| {
                    let rect = ui.available_rect_before_wrap();
                    TextEdit::singleline(&mut self.text_code)
                        .desired_width(rect.width())
                        .ui(ui);
                });

                ui.with_layout(egui::Layout::top_down_justified(Align::LEFT), |ui| {
                    let rect = ui.available_rect_before_wrap();

                    // 画矩形
                    ui.painter().rect(
                        rect,
                        3.0,
                        ui.ctx().style().visuals.window_fill(),
                        Stroke::new(1.0, Color32::BLACK),
                    );

                    // 创建文字
                    let text = format!("  成功:             失败:  ");
                    let text = WidgetText::RichText(RichText::new(text).color(Color32::BLUE));
                    let text = text.into_galley(ui, None, ui.available_width(), TextStyle::Button);

                    // 画文字
                    let text_pos = ui.layout().align_size_within_rect(text.size(), rect).min;
                    text.paint_with_fallback_color(ui.painter(), text_pos, Color32::YELLOW);
                });

                ui.end_row();
            });
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "你好呀！"
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // 设置主题
        ctx.set_visuals(Visuals::light());
        ctx.set_pixels_per_point(10.0);

        // 调试
        ctx.set_debug_on_hover(true);

        // 设置字体
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "DroidSansFallbackFull".to_owned(),
            FontData::from_static(include_bytes!("../fonts/DroidSansFallbackFull.ttf")),
        ); // .ttf and .otf supported
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "DroidSansFallbackFull".to_owned());
        ctx.set_fonts(fonts);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        // 设置像素
        ctx.set_pixels_per_point(2.0);
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}
