#![feature(let_chains)]

use eframe::{
    egui::{self, vec2, Id, Layout, ScrollArea, Ui},
    emath::Align,
    epaint::{pos2, Color32, Rect},
    epi, Frame,
};

pub struct MyApp {}

impl MyApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {}
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::SidePanel::left("left-side")
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "ui.available_rect_before_wrap()ui.available_rect_before_wrap()ui.available_rect_before_wrap()ui.available_rect_before_wrap()ui.available_rect_before_wrap()ui.available_rect_before_wrap():ui.available_rect_before_wrap(): {:?}",
                        ui.available_rect_before_wrap()
                    ));
                });

                ui.vertical_centered(|ui| {
                    // 生成 ui0
                    let mut ui0 = ui.child_ui(
                        Rect::from_center_size(pos2(300.0, 300.0), vec2(300.0, 300.0)),
                        Layout::top_down(Align::Min),
                    );

                    // 给 ui0 画边框
                    ui0.painter().rect_stroke(
                        ui0.available_rect_before_wrap(),
                        0.0,
                        (1.0, Color32::RED),
                    );

                    ui0.horizontal(|ui| {
                        ui.label(format!(
                            "ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap():ui0.available_rect_before_wrap(): {:?}",
                            ui.available_rect_before_wrap()
                        ));

                        ui.painter().rect_stroke(
                            ui.available_rect_before_wrap(),
                            0.0,
                            (1.0, Color32::GOLD),
                        );
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                        ui.label("ui0");
                    });

                    // ui0.vertical_centered(|ui| {
                    //     ui.label("ui0");
                    // });

                    // // 生成 ui1
                    // let mut ui1 = ui0.child_ui(
                    //     Rect::from_center_size(pos2(300.0, 300.0), vec2(100.0, 100.0)),
                    //     Layout::top_down(Align::Min),
                    // );

                    // // 给 ui1 画边框
                    // ui1.painter().rect_stroke(
                    //     ui1.available_rect_before_wrap(),
                    //     0.0,
                    //     (3.0, Color32::RED),
                    // );

                    // ui1.horizontal(|ui| {
                    //     ui.label("ui1");
                    // });

                    // // 生成 ui2
                    // let mut ui2 = ui1.child_ui(
                    //     Rect::from_two_pos(pos2(250.0, 250.0), pos2(550.0, 350.0)),
                    //     Layout::top_down(Align::Min),
                    // );

                    // // 给 ui2 画边框
                    // ui2.painter().rect_stroke(
                    //     ui2.available_rect_before_wrap(),
                    //     0.0,
                    //     (1.0, Color32::BLUE),
                    // );

                    // ui2.horizontal(|ui| {
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    //     ui.label("ui2");
                    // });
                });
            });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(200.0, 150.0)),
        ..Default::default()
    };
    eframe::run_native(
        "hello",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}
