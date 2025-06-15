use skia_safe::textlayout::TextAlign;
use skia_safe::Point;

use crate::core::error::Error;
use crate::utils::{
    builder::{ImageOptions, InputImage},
    encoder::encode_png,
    image::ImageExt,
    text::Text2Image,
    tools::{color_from_hex_code, load_image, local_date, new_paint},
};
use crate::{register_image, text_params};

#[derive(ImageOptions)]
struct Characters {
    #[option(default = "phoenix", choices = ["phoenix", "edgeworth", "godot", "apollo"])]
    character: Option<String>,
}

fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn ace_attorney(
    _: Vec<InputImage>,
    texts: Vec<String>,
    options: Characters,
) -> Result<Vec<u8>, Error> {
    let character = options.character.unwrap();
    let font_families = &["Ace Attorney"];
    let file = load_image(format!("ace_attorney/{character}.png"))?;
    let mut surface = file.to_surface();
    let canvas = surface.canvas();
    let character_text = Text2Image::from_text(
        uppercase_first(&character),
        14.0,
        text_params!(
            font_families = font_families,
            paint = new_paint(color_from_hex_code("#ffffff"))
        ),
    );

    character_text.draw_on_canvas(canvas, Point::new(6.0, 176.0));

    let quote = &texts[0];

    let text = Text2Image::from_text(
        quote,
        14.0,
        text_params!(
            font_families = font_families,
            paint = new_paint(color_from_hex_code("#ffffff")),
            text_align = TextAlign::Left,
            wrap_max_width = 232.0
        ),
    );

    text.draw_on_canvas(canvas, Point::new(7.0, 199.0));

    encode_png(surface.image_snapshot())
}

register_image!(
    "ace_attorney",
    ace_attorney,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["Hello World"],
    date_created = local_date(2025, 6, 12),
    date_modified = local_date(2025, 6, 12)
);
