use eframe::{
    egui::{
        self, vec2, Align, Color32, Direction, FontData, FontDefinitions, FontFamily, RichText,
        Sense, Stroke, TextEdit, TextStyle, Ui, Visuals, Widget, WidgetText,
    },
    epi,
};

fn main() {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(Box::new(MyApp::default()), native_options);
}

#[derive(Default)]
pub struct MyApp {
    text_code: String,
}

impl MyApp {
    pub fn ui(&mut self, ui: &mut Ui) {
        egui::Grid::new("some_unique_id")
            // 长度为2, 第2列宽度是剩余的宽度
            .num_columns(2)
            .show(ui, |ui| {
                ui.set_height(30.0);

                /*
                    第 1 行
                */

                ui.with_layout(egui::Layout::bottom_up(Align::Center), |ui| {
                    // ui.add_sized(vec2(30.0, 80.0), Label::new("测试工位"));

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

                ui.with_layout(
                    egui::Layout::centered_and_justified(Direction::TopDown),
                    |ui| {
                        if ui
                            .button(WidgetText::RichText(
                                RichText::new("PASS").color(Color32::GREEN),
                            ))
                            .clicked()
                        {}
                    },
                );

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
                    let mut rect = ui.available_rect_before_wrap();
                    rect.min.y += 1.0;
                    rect.max.y -= 1.0;

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
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // 设置主题
        ctx.set_visuals(Visuals::light());
        ctx.set_pixels_per_point(10.0);

        // 设置字体
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "DroidSansFallbackFull".to_owned(),
            FontData::from_static(include_bytes!("../fonts/DroidSansFallbackFull.ttf")),
        ); // .ttf and .otf supported
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "DroidSansFallbackFull".to_owned());
        ctx.set_fonts(fonts);
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        // 设置像素
        ctx.set_pixels_per_point(2.0);
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}
