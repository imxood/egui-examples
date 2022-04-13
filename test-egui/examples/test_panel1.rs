#![feature(let_chains)]

use eframe::{egui, epaint::Color32, epi, Frame};

#[derive(Default)]
pub struct MyApp {}

impl MyApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::SidePanel::left("left-panel").show(ctx, |ui| {
            // ui.painter()
            //     .rect_stroke(ui.available_rect_before_wrap(), 0.0, (1.0, Color32::RED));
            // egui::ScrollArea::vertical().show(ui, |ui| if ui.button("left side").clicked() {});

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui.button("left side").clicked() {}
            });

            ui.horizontal(|ui| {
                if ui.button("left side").clicked() {}
                ui.allocate_space(ui.available_size());
            });

            ui.vertical_centered(|ui| if ui.button("right side").clicked() {});
        });
        egui::SidePanel::right("right-panel").show(ctx, |ui| {
            ui.painter()
                .rect_stroke(ui.available_rect_before_wrap(), 0.0, (1.0, Color32::RED));
            ui.horizontal(|ui| if ui.button("right side").clicked() {});
            ui.allocate_space(ui.available_size());
        });
    }
}

fn main() {
    eframe::run_native(
        "hello1",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}
