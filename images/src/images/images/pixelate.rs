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

fn pixelate(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let effect = load_sksl("pixelate.glsl")?;

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let base = 4.0 + ((i as f32) * std::f32::consts::PI / 10.0).sin().abs() * 28.0;
        let pixel_size = [base, base];

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "pixel_size" => {
                    values.extend(pixel_size[0].to_le_bytes());
                    values.extend(pixel_size[1].to_le_bytes());
                }
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
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!("pixelate", pixelate, min_images = 1, max_images = 1);
