use image_derive::ImageOptions;
use skia_safe::{Color, Image};

use crate::{
    core::error::Error,
    register_image,
    utils::{
        builder::InputImage,
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        tools::{load_image, new_surface},
    },
};

#[derive(ImageOptions)]
struct Position {
    #[option(default = "right", choices = ["left", "right", "both"])]
    position: Option<String>,
}

fn bubble_tea(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: Position,
) -> Result<Vec<u8>, Error> {
    let position = options.position.as_deref().unwrap();
    let left = position == "left" || position == "both";
    let right = position == "right" || position == "both";
    let bubble_tea = load_image("bubble_tea/0.png")?;

    let func = |images: Vec<Image>| {
        let frame = images[0].resize_fit((500, 500), Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&frame, (0, 0), None);
        if right {
            canvas.draw_image(&bubble_tea, (0, 0), None);
        }
        if left {
            canvas.draw_image(&bubble_tea.flip_horizontal(), (0, 0), None);
        }
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_image!("bubble_tea", bubble_tea, min_images = 1, max_images = 1);
