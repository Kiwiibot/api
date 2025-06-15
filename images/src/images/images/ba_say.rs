use image_derive::ImageOptions;
use rand::seq::IndexedRandom;
use skia_safe::IRect;

use crate::{
    core::error::Error,
    register_image,
    utils::{
        builder::InputImage, canvas::CanvasExtensions, encoder::encode_png, image::ImageExt,
        tools::load_image,
    },
};

#[derive(ImageOptions)]
struct Position {
    #[option(choices = ["arisu", "izuna", "key", "kokona", "mari", "sena", "yuuka"])]
    character: Option<String>,

    #[option(choices = ["left", "right"])]
    position: Option<String>,
}

fn ba_say(_: Vec<InputImage>, texts: Vec<String>, options: Position) -> Result<Vec<u8>, Error> {
    let character = options.character.as_deref().unwrap_or({
        let mut rng = rand::rng();
        ["arisu", "izuna", "key", "kokona", "mari", "sena", "yuuka"]
            .choose(&mut rng)
            .unwrap()
    });
    let position = options.position.as_deref().unwrap_or_else(|| {
        let mut rng = rand::rng();
        ["left", "right"].choose(&mut rng).unwrap()
    });

    let text = &texts[0];

    let mut frame = load_image(format!("ba_say/{character}.png"))?;
    if position == "left" {
        frame = frame.flip_horizontal();
    }
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let rect = if position == "left" {
        IRect::from_ltrb(60, 0, 580, 200)
    } else {
        IRect::from_ltrb(500, 0, 1020, 200)
    };
    canvas.draw_text_area_auto_font_size(rect, text, 20.0, 100.0, None)?;

    encode_png(surface.image_snapshot())
}

register_image!(
    "bay_say",
    ba_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["What?"]
);
