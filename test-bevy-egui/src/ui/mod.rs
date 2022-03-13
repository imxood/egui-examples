use bevy_egui::egui;
pub mod titlebar_ui;
pub mod setting_ui;
pub mod ui_state;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Window: View {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {

    }
}
