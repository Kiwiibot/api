use core::f32;

use skia_safe::{Data, Image, Paint, SamplingOptions};

use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{
        builder::InputImage,
        encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
        tools::{load_sksl, new_surface},
    },
};

fn wave(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let effect = load_sksl("wave.glsl")?;

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];

        let time = i as f32 * 0.3;
        let amplitude: f32 = 15.0; // pixels
        let frequency: f32 = 0.05; // radians per pixel

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "time" => values.extend(time.to_le_bytes()),
                "amplitude" => values.extend(amplitude.to_le_bytes()),
                "frequency" => values.extend(frequency.to_le_bytes()),
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
            frame_num: 21,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!("wave", wave, min_images = 1, max_images = 1);
