use std::f32::consts::PI;

use image_derive::ImageOptions;

use rand::{Rng, SeedableRng, random, rngs::StdRng};
use skia_safe::{Image, Point};

use crate::{
    core::error::Error,
    register_image, text_params,
    utils::{
        builder::InputImage,
        encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
        image::{Fit, ImageExt},
        string::StringUtils,
        text::Text2Image,
        tools::{color_from_hex_code, load_image, new_paint},
    },
};

#[derive(ImageOptions)]
struct Seed {
    #[option()]
    seed: Option<String>,
}

fn eject(images: Vec<InputImage>, texts: Vec<String>, options: Seed) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let real_seed: Result<u64, Error> = match options.seed {
        Some(seed) => {
            if let Ok(seed) = seed.parse() {
                Ok(seed)
            } else {
                Err(Error::Generic("Failed to parse seed".to_string()))
            }
        }
        None => Ok(random()),
    };

    if let Err(err) = real_seed {
        return Err(err);
    }

    let seed = real_seed.unwrap();

    let mut r = StdRng::seed_from_u64(seed);

    let is_imposter: bool = r.random_bool(0.5);

    let t = format!(
        "{text} was{}An Imposter.",
        if is_imposter { " " } else { " not " }
    );

    let func = |i: usize, images: Vec<Image>| {
        let base = images[0].resize_fit((50, 50), Fit::Contain);

        let to_write = t.clone();

        let frame = load_image(format!("eject/{i:02}.png"))?;
        let f_h = frame.height();
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(frame.clone(), (0, 0), None);
        if i <= 17 {
            let x = (((320 / 15) * i as i32) - 50) as f32;
            let y = ((f_h / 2) - 25) as f32;

            let rotations = 45.0;
            let rotation = (360.0 * rotations / 15.0) * i as f32;
            let angle = -rotation * (PI / 180.0);

            let origin_x = (x + 25f32) as f32;
            let origin_y = (y + 25f32) as f32;

            canvas.save();
            canvas.translate(Point::new(origin_x, origin_y));
            canvas.rotate(angle, None);
            canvas.translate(Point::new(-origin_x, -origin_y));
            canvas.draw_image(&base, Point::new(x, y), None);
            canvas.restore();
        }

        if i > 17 {
            if i <= 27 {
                let letters = ((((t.len() / 10) * (i - 17)) + 1) as f32).ceil() as usize;
                let to_draw = to_write.as_str().slice(..letters + 1);

                let text = Text2Image::from_text(
                    to_draw.to_string(),
                    17.0,
                    text_params!(paint = new_paint(color_from_hex_code("#ffffff"))),
                );
                text.draw_on_canvas(canvas, ((55 - to_draw.len()) as i32, 75));
            } else {
                Text2Image::from_text(
                    &to_write,
                    17.0,
                    text_params!(paint = new_paint(color_from_hex_code("#ffffff"))),
                )
                .draw_on_canvas(canvas, ((55 - to_write.len()) as i32, 75));
            }
        }

        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 52,
            duration: 0.07,
        },
        FrameAlign::NoExtend,
    )
}

register_image!(
    "eject",
    eject,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["User"]
);
