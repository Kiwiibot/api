use skia_safe::Image;

use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{
        builder::InputImage,
        encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
        image::{Fit, ImageExt},
        tools::load_image,
    },
};

fn arona_throw(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let position_list = [
        (270, 295),
        (154, 291),
        (154, 291),
        (89, 211),
        (41, 195),
        (28, 192),
        (16, 200),
        (-10, 206),
        (-40, 210),
        (-80, 214),
        (324, 245),
        (324, 256),
        (331, 251),
        (331, 251),
        (318, 260),
        (318, 260),
    ];
    let position_list2 = [(324, 15), (324, 106), (324, 161), (324, 192)];

    let func = |i: usize, images: Vec<Image>| {
        let pyroxenes = images[0].resize_fit((120, 120), Fit::Cover).circle();
        let arona = load_image(format!("arona_throw/{i:02}.png"))?;
        let mut surface = arona.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&pyroxenes, position_list[i], None);
        if (6..=9).contains(&i) {
            canvas.draw_image(&pyroxenes, position_list2[i - 6], None);
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 16,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!("arona_throw", arona_throw, min_images = 1, max_images = 1,);
