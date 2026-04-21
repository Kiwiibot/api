use skia_safe::Color;

use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{builder::InputImage, encoder::GifEncoder, image::ImageExt, tools::load_image},
};

fn hutao_bite(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [(98, 101, 108, 234), (96, 100, 108, 237)];
    let image = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..2 {
        let frame = load_image(format!("hutao_bite/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (w, h, x, y) = locs[i];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_image!("hutao_bite", hutao_bite, min_images = 1, max_images = 1,);
