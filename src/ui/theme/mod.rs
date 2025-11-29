pub mod color;
pub mod palette;
pub mod settings;
pub mod settings_ui;

pub mod compat;
pub mod material_colors;
pub mod extractor;
pub mod wallpaper_monitor;
pub mod context;
pub mod access;
pub mod animation;
pub mod manager;

pub use access::*;
pub use animation::*;
pub use color::*;
pub use compat::{setup_theme, Theme};
pub use context::*;
pub use extractor::*;
pub use material_colors::*;
pub use palette::*;
pub use settings::*;
// pub use settings_ui::*;

pub use wallpaper_monitor::*;
