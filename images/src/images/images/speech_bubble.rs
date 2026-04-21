use image_derive::ImageOptions;
use skia_safe::{BlendMode, Image, Paint};

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
struct SpeechBubbleOptions {
    /// The tail direction of the speech bubble.
    #[option(default = "left", choices = ["left", "right", "centre"])]
    direction: Option<String>,

    /// The shape of the speech bubble.
    #[option(default = "round", choices = ["round", "rectangle"])]
    shape: Option<String>,

    /// Whether to use the big tail variant.
    #[option(default = false)]
    big: Option<bool>,

    /// The background of the speech bubble.
    #[option(default = "transparent", choices = ["transparent", "none", "white", "image"])]
    background: Option<String>,

    /// Whether to flip the speech bubble vertically.
    #[option(default = false)]
    flip: Option<bool>,
}

fn speech_bubble(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: SpeechBubbleOptions,
) -> Result<Vec<u8>, Error> {
    let direction = options.direction.as_deref().unwrap();
    let shape = options.shape.as_deref().unwrap();
    let big = options.big.unwrap_or(false);
    let background = options.background.as_deref().unwrap();
    let flip = options.flip.unwrap_or(false);

    let prefix = match (shape, big) {
        ("rectangle", true) => "rectangle_big_tail",
        ("rectangle", false) => "rectangle_tail",
        (_, true) => "big_tail",
        _ => "tail",
    };

    let bubble = load_image(format!("speech_bubble/{}_{}.png", prefix, direction))?;

    let func = |images: Vec<Image>| {
        let img = &images[0];
        let img_w = img.width();
        let img_h = img.height();

        // Resize bubble to match image width, preserving aspect ratio
        let bubble_resized = bubble.resize_width(img_w);
        let bubble_h = bubble_resized.height();

        let bubble_final = if flip {
            bubble_resized.flip_vertical()
        } else {
            bubble_resized
        };

        // Position: top by default, bottom if flipped
        let bubble_y = if flip { img_h - bubble_h } else { 0 };

        let mut surface = new_surface((img_w, img_h));
        let canvas = surface.canvas();

        // Draw the base image
        canvas.draw_image(img, (0, 0), None);

        match background {
            "none" => {
                // "none": use Multiply blend mode so that white bubble fill
                // leaves the image unchanged, while the black outline is drawn on top
                let mut paint = Paint::default();
                paint.set_blend_mode(BlendMode::Multiply);
                canvas.draw_image(&bubble_final, (0, bubble_y), Some(&paint));
            }
            "white" => {
                // Draw bubble as-is (white fill + outline) on top of the image
                canvas.draw_image(&bubble_final, (0, bubble_y), None);
            }
            "image" if images.len() > 1 => {
                // Use the bubble shape as a mask for the second input image
                let overlay = images[1].resize_fit(bubble_final.dimensions(), Fit::Cover);
                let masked = overlay.clip_mask(&bubble_final);
                canvas.draw_image(&masked, (0, bubble_y), None);
            }
            _ => {
                // Cut the bubble shape out of the image (truly transparent hole)
                let mut paint = Paint::default();
                paint.set_blend_mode(BlendMode::DstOut);
                canvas.draw_image(&bubble_final, (0, bubble_y), Some(&paint));
            }
        }

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_image!(
    "speech_bubble",
    speech_bubble,
    min_images = 1,
    max_images = 2,
);
