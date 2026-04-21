use skia_safe::{Data, IRect, Image, Paint};

use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{
        builder::InputImage,
        encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
        tools::{default_sampling_options, load_sksl, new_surface},
    },
};

fn sphere_rotate(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let total_frames = 60;

    let effect = load_sksl("sphere_rotate.glsl")?;

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let (canvas_w, canvas_h) = (300, 300);
        let mut surface = new_surface((canvas_w, canvas_h));
        let canvas = surface.canvas();

        let angle = i as f32 / total_frames as f32 * std::f32::consts::PI * 2.0;

        let mut values = vec![];

        for uniform in effect.uniforms() {
            match uniform.name() {
                "angle" => values.extend(angle.to_le_bytes()),
                "canvas_size" => {
                    values.extend((canvas_w as f32).to_le_bytes());
                    values.extend((canvas_h as f32).to_le_bytes());
                }
                "image_size" => {
                    values.extend((img.width() as f32).to_le_bytes());
                    values.extend((img.height() as f32).to_le_bytes());
                }
                _ => {}
            }
        }

        let uniforms = Data::new_copy(&values);

        let image_shader = img
            .to_shader(None, default_sampling_options(), None)
            .unwrap();
        let shader = effect
            .make_shader(uniforms, &[image_shader.into()], None)
            .unwrap();

        let mut paint = Paint::default();
        paint.set_shader(shader);
        canvas.draw_irect(IRect::from_xywh(0, 0, canvas_w, canvas_h), &paint);

        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: total_frames,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!(
    "sphere_rotate",
    sphere_rotate,
    min_images = 1,
    max_images = 1
);
