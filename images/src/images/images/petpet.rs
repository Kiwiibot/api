use skia_safe::Image;

use crate::{
    core::error::Error, images::options::Circle, register_image, utils::{
        builder::InputImage, encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo}, image::ImageExt, tools::{load_image, new_surface}
    }
};

fn petpet(images: Vec<InputImage>, _: Vec<String>, options: Circle) -> Result<Vec<u8>, Error> {
    let locs = [
        (14, 20, 98, 98),
        (12, 33, 101, 85),
        (8, 40, 110, 76),
        (10, 33, 102, 84),
        (12, 20, 98, 98),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let hand = load_image(format!("petpet/{i}.png"))?;
        let mut surface = new_surface(hand.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
        let image = images[0].square();
        let image = if options.circle.unwrap() {
            image.circle()
        } else {
            image
        };
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&hand, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!("petpet", petpet, min_images = 1, max_images = 1);
