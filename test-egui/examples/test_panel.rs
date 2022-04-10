#![feature(let_chains)]

use eframe::{
    egui::{self, vec2, Id, ScrollArea, Ui},
    epaint::Color32,
    epi,
};

pub struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "你好呀!"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::TopBottomPanel::top("top-panel").show(ctx, |ui| {
            if ui.button("top side").clicked() {
                ui.label("clicked");
            }
        });
        egui::TopBottomPanel::bottom("bottom-panel").show(ctx, |ui| {
            if ui.button("bottom side").clicked() {
                ui.label("clicked");
            }
        });
        egui::SidePanel::left("left-panel").show(ctx, |ui| {
            if ui.button("left side").clicked() {
                ui.label("clicked");
            }
            ui.horizontal(|ui| {
                ui.painter()
                    .rect_stroke(ui.available_rect_before_wrap(), 0.0, (1.0, Color32::RED));
            });
            // ui.vertical_centered(|ui| {
            // });
        });
        egui::SidePanel::right("right-panel").show(ctx, |ui| {
            ui.painter()
                .rect_stroke(ui.available_rect_before_wrap(), 0.0, (1.0, Color32::RED));
            ui.vertical_centered(|ui| {
                if ui.button("right side").clicked() {
                    ui.label("clicked");
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top1-panel").show(ctx, |ui| {
                if ui.button("top1 side").clicked() {
                    ui.label("clicked");
                }
            });
            egui::TopBottomPanel::bottom("bottom1-panel").show(ctx, |ui| {
                if ui.button("bottom1 side").clicked() {
                    ui.label("clicked");
                }
            });
            egui::SidePanel::left("left1-panel").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    if ui.button("left1 side").clicked() {
                        ui.label("clicked");
                    }
                });
            });
            egui::SidePanel::right("right1-panel").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    if ui.button("right1 side").clicked() {
                        ui.label("clicked");
                    }
                });
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                if ui.button("center1").clicked() {
                    ui.label("clicked");
                }
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
