
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

            let rotations = 3.0;
            let rotation = (360.0 * rotations / 17.0) * i as f32;

            let origin_x = (x + 25f32) as f32;
            let origin_y = (y + 25f32) as f32;

            canvas.save();
            canvas.translate(Point::new(origin_x, origin_y));
            canvas.rotate(-rotation, None);
            canvas.translate(Point::new(-origin_x, -origin_y));
            canvas.draw_image(&base, Point::new(x, y), None);
            canvas.restore();
        }

        if i > 17 {
            let to_draw = if i <= 27 {
                let letters = ((t.len() as f32 / 10.0) * (i - 17) as f32).ceil() as usize;
                to_write.as_str().slice(..letters.min(t.len())).to_string()
            } else {
                to_write.clone()
            };

            let text = Text2Image::from_text(
                &to_draw,
                17.0,
                text_params!(paint = new_paint(color_from_hex_code("#ffffff"))),
            );
            let text_width = text.longest_line();
            let x = ((320.0 - text_width) / 2.0) as i32;
            text.draw_on_canvas(canvas, (x, 75));
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
