use skia_safe::Image;

use crate::{
    core::error::Error, images::options::NoOptions, register_image, utils::{
        builder::InputImage,
        encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
        image::ImageExt,
        tools::load_image,
    }
};

fn homelander(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, _: Vec<Image>| {
        let base = load_image(format!("homelander/frames/Homie-{:04}.png", i + 1))?;
        let mut surface = base.to_surface();

        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 1045,
            duration: 0.04,
        },
        FrameAlign::NoExtend,
    )
}

register_image!("homelander", homelander, min_images = 1, max_images = 1);
