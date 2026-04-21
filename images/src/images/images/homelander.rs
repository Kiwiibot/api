use std::{fs, sync::LazyLock};

use serde::{Deserialize, Serialize};

use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{
        builder::InputImage,
        config::IMAGES_DIR,
        decoder::CodecExtensions,
        encoder::VideoEncoder,
        image::{Fit, ImageExt},
        tools::{load_image, new_surface},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Frame {
    file: String,
    show: bool,
    bbox: Option<[i32; 4]>,
}

static FRAMES: LazyLock<Vec<Frame>> = LazyLock::new(|| {
    serde_json::from_str(
        fs::read_to_string(IMAGES_DIR.join("homelander/frames.json"))
            .expect("frames.json file")
            .as_str(),
    )
    .expect("Valid frames")
});

fn homelander(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let mut images = images
        .into_iter()
        .map(|image| image.codec)
        .collect::<Vec<_>>();

    let user_img = images[0].first_frame()?;

    // Get frame dimensions from the first frame and pre-resize user image once
    let first_frame = load_image(format!("homelander/frames/{}", FRAMES[0].file))?;
    let frame_w = first_frame.width();
    let frame_h = first_frame.height();
    
    // The maximum dimension of the transparent movie screen across all frames is 464x345
    let screen_w = 464;
    let screen_h = 345;
    let user_img_resized = user_img.resize_fit((screen_w, screen_h), Fit::Cover);

    let fps = 25.0;
    let mut encoder = VideoEncoder::new(fps);

    for frame_info in FRAMES.iter() {
        let base = load_image(format!("homelander/frames/{}", frame_info.file))?;

        if frame_info.show {
            // Draw user image behind, then frame on top
            let mut surface = new_surface((frame_w, frame_h));
            let canvas = surface.canvas();

            if let Some(bbox) = &frame_info.bbox {
                // Calculate center of the current screen bounding box
                let center_x = (bbox[0] + bbox[2]) as f32 / 2.0;
                let center_y = (bbox[1] + bbox[3]) as f32 / 2.0;
                
                // Draw pre-resized user image perfectly centered in the bounding box
                let draw_x = center_x - (screen_w as f32 / 2.0);
                let draw_y = center_y - (screen_h as f32 / 2.0);
                canvas.draw_image(&user_img_resized, (draw_x, draw_y), None);
            }

            // Draw the frame on top (transparency reveals user image)
            canvas.draw_image(&base, (0, 0), None);

            encoder.add_frame(surface.image_snapshot())?;
        } else {
            // No transparency — frame is fully opaque, use directly
            encoder.add_frame(base)?;
        }
    }

    encoder.finish()
}

// yeah uh, registering a video as an image is questionable
register_image!("homelander", homelander, min_images = 1, max_images = 1);
