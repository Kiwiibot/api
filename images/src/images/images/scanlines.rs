use core::f32;
use skia_safe::{Data, Image, Paint, SamplingOptions};

use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{
        builder::InputImage,
        encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
        tools::{load_sksl, new_surface},
    },
};

fn scanlines(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let effect = load_sksl("scanlines.glsl")?;

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let frame_num = 36;
        let time = i as f32 / frame_num as f32;

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "time" => values.extend(time.to_le_bytes()),
                _ => {}
            }
        }
        let uniforms = Data::new_copy(&values);

        let image_shader = img
            .to_shader(None, SamplingOptions::default(), None)
            .unwrap();
        let shader = effect
            .make_shader(&uniforms, &[image_shader.into()], None)
            .unwrap();

        let mut surface = new_surface(img.dimensions());
        let canvas = surface.canvas();
        let mut paint = Paint::default();
        paint.set_shader(shader);
        canvas.draw_paint(&paint);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 18,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!("scanlines", scanlines, min_images = 1, max_images = 1);
