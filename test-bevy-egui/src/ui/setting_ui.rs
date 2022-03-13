use bevy_egui::egui::{Align2, Context, Window};

pub struct SettingWindow {
    /// 控制窗口显示
    open: bool,
    /// 标记着 控制窗口第一次打开
    first_open: bool,
}

impl Default for SettingWindow {
    fn default() -> Self {
        Self {
            open: false,
            first_open: true,
        }
    }
}

impl SettingWindow {
    pub fn show(&mut self, ctx: &Context) {
        let open = self.open;
        let window = Window::new("setting")
            .collapsible(false)
            .open(&mut self.open);
        // 如果是第一次打开, 就设置居中
        let window = if open && self.first_open {
            println!("first_open");
            window.anchor(Align2::CENTER_CENTER, [0.0, -30.0])
        } else {
            window
        };

        window.show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("settings");
            });
        });

        if open && self.first_open {
            ctx.request_repaint();
            self.first_open = false;
        }
    }

    pub fn trigger_show(&mut self) {
        self.open = !self.open;
        if self.open {
            self.first_open = true;
        }
    }
}
