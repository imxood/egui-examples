#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epi, Frame};
use egui::{
    epaint::{Mesh, Vertex},
    vec2, Color32, Pos2, Rect, Sense, Shape,
};

#[derive(Default)]
struct MyApp {}

impl MyApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("test paint UV");

            // 画一个 铺满有效区域 的矩形
            let (_res, painter) =
                ui.allocate_painter(ui.available_rect_before_wrap().size(), Sense::click());
            let rect = painter.clip_rect();
            let mut mesh = Mesh::default();
            mesh.add_colored_rect(rect, Color32::LIGHT_BLUE);

            // 画一个 小矩形

            let left_top = rect.left_top();
            let right_bottom = left_top + vec2(100.0, 100.0);
            let rect = Rect::from_min_max(left_top, right_bottom);
            mesh.add_colored_rect(rect, Color32::LIGHT_YELLOW);

            // 画另一个 矩形

            // 连续的顶点索引构成一个完整的形状， 每3个三角形构成一个形状, 矩形需要两个三角形
            let idx = mesh.vertices.len() as u32;
            let rect = rect.translate(vec2(100.0, 100.0));
            println!("rect: {:?}", &rect);
            mesh.indices
                .extend([idx, idx + 1, idx + 2, idx, idx + 3, idx + 2]);
            mesh.vertices.extend([
                Vertex {
                    pos: rect.left_top(),
                    uv: Pos2::ZERO,
                    color: Color32::LIGHT_RED,
                },
                Vertex {
                    pos: rect.right_top(),
                    uv: Pos2::ZERO,
                    color: Color32::LIGHT_RED,
                },
                Vertex {
                    pos: rect.right_bottom(),
                    uv: Pos2::ZERO,
                    color: Color32::LIGHT_RED,
                },
                Vertex {
                    pos: rect.left_bottom(),
                    uv: Pos2::ZERO,
                    color: Color32::LIGHT_RED,
                },
            ]);

            // 画一个 线段

            let rect = rect.translate(vec2(100.0, 100.0));
            let left_top = rect.left_top();
            let right_bottom = left_top + vec2(100.0, 1.0);
            let rect = Rect::from_min_max(left_top, right_bottom);
            mesh.add_colored_rect(rect, Color32::LIGHT_YELLOW);

            painter.add(Shape::Mesh(mesh));
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "hello",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}
