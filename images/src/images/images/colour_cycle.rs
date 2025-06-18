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

fn colour_cycle(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let effect = load_sksl("colour_cycle.glsl")?;

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let hue_shift = (i as f32) / 16.0;

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "hue_shift" => values.extend(hue_shift.to_le_bytes()),
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
            frame_num: 16,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!("colour_cycle", colour_cycle, min_images = 1, max_images = 1);
