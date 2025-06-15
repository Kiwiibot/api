use skia_safe::{Color, IRect};

use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image, text_params,
    utils::{
        builder::InputImage,
        canvas::CanvasExtensions,
        encoder::encode_png,
        image::ImageExt,
        tools::{load_image, new_paint},
    },
};

fn bronya_holdsign(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("bronya_holdsign/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(190, 675, 640, 930),
        text,
        25.0,
        60.0,
        text_params!(paint = new_paint(Color::from_rgb(111, 95, 95))),
    )?;
    encode_png(surface.image_snapshot())
}

register_image!(
    "bronya_holdsign",
    bronya_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["Real"]
);
