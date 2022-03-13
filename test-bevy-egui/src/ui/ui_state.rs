use bevy::window::WindowMode;

use super::{setting_ui::SettingWindow, titlebar_ui::Titlebar};
use crate::resources::theme::Theme;

pub struct UiState {
    pub maximized: bool,
    pub window_mode: WindowMode,
    pub scale_factor: f64,
    pub theme: Theme,
    pub titlebar: Titlebar,
    pub setting_window: SettingWindow,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            maximized: true,
            window_mode: WindowMode::Windowed,
            scale_factor: 1.25,
            theme: Theme::default(),
            titlebar: Titlebar::default(),
            setting_window: Default::default(),
        }
    }
}
