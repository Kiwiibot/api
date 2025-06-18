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

fn prism(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let effect = load_sksl("prism.glsl")?;

    let frame_num = 32;
    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let x = img.width() as f32 * 0.5;
        let y = img.height() as f32 * 0.5;
        let radius = (x * x + y * y).sqrt() * 0.8;

        let t = i as f32 / frame_num as f32;
        let anim = (std::f32::consts::PI * 2.0 * t).sin() * 0.5 + 0.5;
        let zoom = 0.7 + 0.3 * anim;
        let strength = 18.0 * (1.0 - anim);

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "center" => {
                    values.extend(x.to_le_bytes());
                    values.extend(y.to_le_bytes());
                }
                "radius" => values.extend(radius.to_le_bytes()),
                "strength" => values.extend(strength.to_le_bytes()),
                "zoom" => values.extend(zoom.to_le_bytes()),
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
            frame_num: frame_num,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!("prism", prism, min_images = 1, max_images = 1);
