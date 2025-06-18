use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use skia_safe::{Data, Image, Paint, SamplingOptions};

use crate::utils::tools::load_sksl;
use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{
        builder::InputImage,
        encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
        tools::new_surface,
    },
};

fn jitter(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let effect = load_sksl("jitter.glsl")?;

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let mut rng = StdRng::seed_from_u64(i as u64 * 12345);
        let dx = (rng.random::<f32>() - 0.5) * 10.0;
        let dy = (rng.random::<f32>() - 0.5) * 10.0;

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "offset" => {
                    values.extend(dx.to_le_bytes());
                    values.extend(dy.to_le_bytes());
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
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!("jitter", jitter, min_images = 1, max_images = 1);