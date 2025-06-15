use skia_safe::{Color, Image};

use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{
        builder::InputImage,
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        tools::{load_image, new_surface},
    },
};

fn blood_pressure(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("blood_pressure/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((414, 450), Fit::Cover);
        canvas.draw_image(&image, (16, 17), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_image!(
    "blood_pressure",
    blood_pressure,
    min_images = 1,
    max_images = 1
);
