use std::f32::consts::PI;

use skia_safe::Image;

use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{
        builder::InputImage,
        encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
        tools::{local_date, new_surface},
    },
};

fn bounce(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let fc = 15;

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let w = img.width();
        let h = img.height();

        let hh = (h as f32 / 2.0).round() as i32;
        let m = PI / fc as f32;

        let o = (hh as f32 * (-((i as f32) * m).sin() + 1.0)).round() as i32;

        let ch = h + hh;
        let mut surface = new_surface((w, ch));

        let canvas = surface.canvas();

        canvas.draw_image(img, (0, o), None);

        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: fc,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!(
    "bounce",
    bounce,
    min_images = 1,
    max_images = 1,
    date_created = local_date(2025, 06, 14),
    date_modified = local_date(2025, 06, 14)
);
