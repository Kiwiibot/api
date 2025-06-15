use std::path::PathBuf;
use std::sync::LazyLock;

use serde::Deserialize;

pub const IMAGES_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("/data/resources/images"));
pub const FONTS_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("/data/resources/fonts"));

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