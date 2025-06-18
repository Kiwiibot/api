use rand::rngs::StdRng;
use rand::{SeedableRng, seq::SliceRandom};
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

const BLOCKS_X: usize = 6;
const BLOCKS_Y: usize = 6;
const BLOCK_COUNT: usize = BLOCKS_X * BLOCKS_Y;

fn random_block_shuffle(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let effect = load_sksl("random_block_shuffle.glsl")?;

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let image_size = [img.width() as f32, img.height() as f32];

        let mut indices: Vec<i32> = (0..BLOCK_COUNT as i32).collect();
        let mut rng = StdRng::seed_from_u64(i as u64 * 987654321);
        indices.shuffle(&mut rng);

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "image_size" => {
                    values.extend(image_size[0].to_le_bytes());
                    values.extend(image_size[1].to_le_bytes());
                }
                "block_map" => {
                    for idx in &indices {
                        values.extend(idx.to_le_bytes());
                    }
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
            frame_num: 14,
            duration: 0.08,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!(
    "random_block_shuffle",
    random_block_shuffle,
    min_images = 1,
    max_images = 1
);
