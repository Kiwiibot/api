use skia_safe::{Color, Image};

use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{
        builder::InputImage,
        encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
        image::ImageExt,
        tools::{load_image, new_surface},
    },
};

fn capoo_point(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (165, 167, 57, 290),
        (165, 167, 53, 290),
        (160, 165, 57, 293),
        (165, 167, 56, 290),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let image = images[0].square();
        let frame = load_image(format!("capoo_point/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (w, h, x, y) = locs[i];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);

        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            duration: 0.1,
            frame_num: 4,
        },
        FrameAlign::NoExtend,
    )
}

register_image!("capoo_point", capoo_point, min_images = 1, max_images = 1);
