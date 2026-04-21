use std::{fs, sync::LazyLock};

use image_derive::ImageOptions;
use serde::{Deserialize, Serialize};
use skia_safe::Image;

use crate::{
    core::error::Error,
    register_image, text_params,
    utils::{
        builder::InputImage,
        config::IMAGES_DIR,
        encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
        image::ImageExt,
        text::Text2Image,
        tools::{color_from_hex_code, load_image, new_paint},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Frame {
    corners: [[f32; 2]; 3],
    file: String,
    show: bool,
}

static FRAMES: LazyLock<Vec<Frame>> = LazyLock::new(|| {
    serde_json::from_str(
        fs::read_to_string(IMAGES_DIR.join("illegal/frames.json"))
            .expect("File")
            .as_str(),
    )
    .expect("Valid frames")
});

#[derive(ImageOptions)]
struct Verb {
    #[option(default = "is", choices = ["is", "are", "am"])]
    verb: Option<String>,
}

fn illegal(_: Vec<InputImage>, texts: Vec<String>, options: Verb) -> Result<Vec<u8>, Error> {
    let verb = options
        .verb
        .unwrap_or_else(|| "is".to_string())
        .to_uppercase();
    let text = &texts[0];

    let func = |i: usize, _: Vec<Image>| -> Result<Image, Error> {
        let frame = &FRAMES[i];
        let img = load_image(format!("illegal/{}", frame.file))?;
        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        if frame.show {
            let corners = frame.corners;
            let x = corners[0][0];
            let y = corners[0][1];

            let text = Text2Image::from_text(
                format!("{text}\n{verb} NOW ILLEGAL"),
                20.0,
                text_params!(
                    font_families = &["Impact"],
                    paint = new_paint(color_from_hex_code("#000000")),
                    wrap_max_width = 50f32
                ),
            );

            text.draw_on_canvas(canvas, (x, y));
        }

        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        vec![],
        func,
        GifInfo {
            frame_num: FRAMES.len() as u32,
            duration: 0.07,
        },
        FrameAlign::NoExtend,
    )
}

register_image!(
    "illegal",
    illegal,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["Skibidi toilet"]
);
