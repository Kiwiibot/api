use image_derive::ImageOptions;
use rand::seq::IndexedRandom;
use skia_safe::{Color, FontStyle, IRect};

use crate::{
    core::error::Error,
    register_image, text_params,
    utils::{
        builder::InputImage,
        canvas::CanvasExtensions,
        encoder::encode_png,
        image::ImageExt,
        tools::{load_image, new_paint},
    },
};

#[derive(ImageOptions)]
struct Expression {
    #[option(choices = ["angry", "black", "happy", "shy", "speechless"])]
    expression: Option<String>,
}

fn anan_say(_: Vec<InputImage>, texts: Vec<String>, options: Expression) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let expression = options.expression.as_deref().unwrap_or_else(|| {
        let mut rng = rand::rng();
        ["angry", "black", "happy", "shy", "speechless"]
            .choose(&mut rng)
            .unwrap()
    });

    let base_image = load_image(&format!("anan_say/{}.png", expression))?;
    let mut surface = base_image.to_surface();
    let canvas = surface.canvas();

    let text_rect = IRect::from_ltrb(105, 445, 412, 625);
    canvas.draw_text_area_auto_font_size(
        text_rect,
        text,
        20.0,
        60.0,
        text_params!(
            paint = new_paint(Color::BLACK),
            font_style = FontStyle::bold()
        ),
    )?;

    let hand_image = load_image("anan_say/hand.png")?;
    canvas.draw_image(&hand_image, (0, 0), None);

    encode_png(surface.image_snapshot())
}

register_image!("anan_say", anan_say, min_texts = 1, max_texts = 1);
