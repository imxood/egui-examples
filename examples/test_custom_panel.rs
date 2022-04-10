#![feature(let_chains)]

use std::ops::RangeInclusive;

use eframe::{
    egui::{self, vec2, Context, Frame, Id, InnerResponse, LayerId, Layout, ScrollArea, Ui},
    emath::Align,
    epaint::{pos2, Color32, Pos2, Rect, Stroke},
    epi,
};

struct Panel {
    id: Id,
    frame: Option<Frame>,
    resizable: bool,
    rect: Rect,
    width_range: RangeInclusive<f32>,
    height_range: RangeInclusive<f32>,
}

impl Panel {
    pub fn new(id_source: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id_source),
            frame: None,
            resizable: true,
            rect: Rect::from_min_size(Pos2::ZERO, vec2(200.0, 200.0)),
            width_range: 96.0..=f32::INFINITY,
            height_range: 96.0..=f32::INFINITY,
        }
    }

    pub fn set_pos(mut self, pos: Rect) -> Self {
        self.rect = pos;
        self
    }

    /// Change the background color, margins, etc.
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = Some(frame);
        self
    }

    pub fn show<'c, R>(
        self,
        ctx: &Context,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let layer_id = LayerId::background();
        let rect = self.rect.intersect(ctx.available_rect());
        let rect = rect.intersect(ctx.input().screen_rect());
        println!("rect: {:?}", rect);

        // 背景 ui
        let mut background_ui = Ui::new(ctx.clone(), layer_id, self.id, rect, rect);
        // 画边框
        background_ui
            .painter()
            .rect_stroke(rect, 0.0, (1.0, Color32::RED));

        let mut child_ui =
            background_ui.child_ui_with_id_source(rect, Layout::top_down(Align::Center), self.id);

        child_ui.horizontal(|ui| add_contents(ui))

        // 面板 ui
        // let mut panel_ui =
        //     background_ui.child_ui_with_id_source(rect, Layout::top_down(Align::Min), id);
    }
}

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
        Panel::new("custom_panel")
            .set_pos(Rect::from_min_size(pos2(100.0, 100.0), vec2(200.0, 100.0)))
            .show(ctx, |ui| {
                if ui.button("hello").clicked() {
                    ui.label("clicked");
                }
            });
        // egui::SidePanel::left("left-panel").show(ctx, |ui| {
        //     if ui.button("left side").clicked() {
        //         ui.label("clicked");
        //     }
        //     ui.horizontal(|ui| {
        //         ui.painter()
        //             .rect_stroke(ui.available_rect_before_wrap(), 0.0, (1.0, Color32::RED));
        //     });
        //     // ui.vertical_centered(|ui| {
        //     // });
        // });
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
