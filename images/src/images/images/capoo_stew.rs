use skia_safe::Image;

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

fn capoo_stew(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("capoo_stew/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let image = images[0].circle().resize_exact((80, 80));
        let y = if [2, 3, 5].contains(&i) { 45 } else { 47 };
        canvas.draw_image(&image, (88, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.08,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!("capoo_stew", capoo_stew, min_images = 1, max_images = 1);
