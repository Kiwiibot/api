use std::path::PathBuf;
use std::sync::LazyLock;

use serde::Deserialize;

pub const DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::var("DATA_DIR")
        .unwrap_or_else(|_| "/data".to_string())
        .into()
});

pub const IMAGES_DIR: LazyLock<PathBuf> = LazyLock::new(|| DATA_DIR.join("resources/images"));
pub const FONTS_DIR: LazyLock<PathBuf> = LazyLock::new(|| DATA_DIR.join("resources/fonts"));
pub const SKSL_DIR: LazyLock<PathBuf> = LazyLock::new(|| DATA_DIR.join("glsl"));

pub static FONT_CONFIG: LazyLock<FontConfig> = LazyLock::new(|| FontConfig::default());

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct FontConfig {
    pub use_local_fonts: bool,
    pub default_font_families: Vec<String>,
}

impl Default for FontConfig {
    fn default() -> Self {
        FontConfig {
            use_local_fonts: true,
            default_font_families: vec!["Noto Sans SC", "Noto Color Emoji"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}
