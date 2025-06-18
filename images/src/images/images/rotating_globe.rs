use crate::{
    core::error::Error,
    images::options::NoOptions,
    register_image,
    utils::{
        builder::InputImage,
        encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
        tools::{load_image, new_surface},
    },
};
use skia_safe::{
    AlphaType, ColorType, Image, ImageInfo, image::CachingHint, images::raster_from_data,
};

fn rotating_globe(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let globe_size: usize = 500;
    let n_frames: usize = 32;

    let diff_mask_img = load_image("globe/globediffuse.png")?;
    let spec_mask_img = load_image("globe/globespec.png")?;
    let sphere_map = load_image("globe/spheremap.png")?;

    let mask_w = diff_mask_img.width();
    let mask_h = diff_mask_img.height();
    let mask_info = ImageInfo::new(
        (mask_w, mask_h),
        ColorType::BGRA8888,
        AlphaType::Unpremul,
        None,
    );
    let mut diff_mask_pixels = vec![0u8; (mask_w * mask_h * 4) as usize];
    diff_mask_img.read_pixels(
        &mask_info,
        &mut diff_mask_pixels,
        (mask_w * 4) as usize,
        (0, 0),
        CachingHint::Allow,
    );
    let mut spec_mask_pixels = vec![0u8; (mask_w * mask_h * 4) as usize];
    spec_mask_img.read_pixels(
        &mask_info,
        &mut spec_mask_pixels,
        (mask_w * 4) as usize,
        (0, 0),
        CachingHint::Allow,
    );

    let sphere_w = sphere_map.width();
    let sphere_h = sphere_map.height();
    let sphere_info = ImageInfo::new(
        (sphere_w, sphere_h),
        ColorType::BGRA8888,
        AlphaType::Unpremul,
        None,
    );
    let mut sphere_pixels = vec![0u8; (sphere_w * sphere_h * 4) as usize];
    sphere_map.read_pixels(
        &sphere_info,
        &mut sphere_pixels,
        (sphere_w * 4) as usize,
        (0, 0),
        CachingHint::Allow,
    );

    let tex_zoom = 1.4;

    let func = move |fi: usize, imgs: Vec<Image>| {
        let world_img = &imgs[0];

        let world_w = world_img.width();
        let world_h = world_img.height();
        let world_info = ImageInfo::new(
            (world_w, world_h),
            ColorType::BGRA8888,
            AlphaType::Unpremul,
            None,
        );
        let mut world_pixels = vec![0u8; (world_w * world_h * 4) as usize];
        world_img.read_pixels(
            &world_info,
            &mut world_pixels,
            (world_w * 4) as usize,
            (0, 0),
            CachingHint::Allow,
        );

        let mut surface = new_surface((globe_size as i32, globe_size as i32));
        let canvas = surface.canvas();
        canvas.clear(skia_safe::Color::TRANSPARENT);

        let rot = 2.0 * std::f32::consts::PI * (fi as f32) / (n_frames as f32 + 12.0);

        let cx = globe_size as f32 * 0.5;
        let cy = globe_size as f32 * 0.5;
        let r = globe_size as f32 * 0.48;

        let mut pixels = vec![0u8; globe_size * globe_size * 4];
        for y in 0..globe_size {
            for x in 0..globe_size {
                let xf = x as f32 + 0.5;
                let yf = y as f32 + 0.5;
                let nx = (xf - cx) / r;
                let ny = (yf - cy) / r;
                let rr = nx * nx + ny * ny;
                let offset = (y * globe_size + x) * 4;
                if rr > 1.0 {
                    pixels[offset + 0] = 0;
                    pixels[offset + 1] = 0;
                    pixels[offset + 2] = 0;
                    pixels[offset + 3] = 0;
                    continue;
                }
                let nz = (1.0 - rr).sqrt();
                let (sx, sy, sz) = (nx, ny, nz);

                let theta = rot - sx.atan2(sz);
                let phi = sy.asin();

                let u = 0.5 + theta / (2.0 * std::f32::consts::PI) * tex_zoom;
                let v = 0.5 + (phi / std::f32::consts::PI) * tex_zoom;

                let (uw, vh) = (world_w as f32, world_h as f32);
                let mut iu = ((u.fract() + 1.0) % 1.0) * uw;
                let mut iv = ((v.fract() + 1.0) % 1.0) * vh;
                if iu >= uw {
                    iu = uw - 1.0;
                }
                if iv >= vh {
                    iv = vh - 1.0;
                }
                let (ix, iy) = (iu as usize, iv as usize);
                let idx = ((iy * world_w as usize + ix) * 4) as usize;
                let b = world_pixels[idx + 0];
                let g = world_pixels[idx + 1];
                let r = world_pixels[idx + 2];

                let mask_x = x.min(mask_w as usize - 1);
                let mask_y = y.min(mask_h as usize - 1);
                let midx = ((mask_y * mask_w as usize + mask_x) * 4) as usize;
                let mb = diff_mask_pixels[midx + 0];
                let mg = diff_mask_pixels[midx + 1];
                let mr = diff_mask_pixels[midx + 2];
                let sidx = ((mask_y * mask_w as usize + mask_x) * 4) as usize;
                let sb = spec_mask_pixels[sidx + 0];
                let sg = spec_mask_pixels[sidx + 1];
                let sr = spec_mask_pixels[sidx + 2];

                let mapped_r = r as f32;
                let mapped_g = g as f32;
                let mapped_b = b as f32;
                let dr = mr as f32 / 255.0;
                let dg = mg as f32 / 255.0;
                let db = mb as f32 / 255.0;
                let sr = sr as f32;
                let sg = sg as f32;
                let sb = sb as f32;

                let mut out_r = (mapped_r * dr + sr).min(255.0);
                let mut out_g = (mapped_g * dg + sg).min(255.0);
                let mut out_b = (mapped_b * db + sb).min(255.0);

                let view = [0.0, 0.0, 1.0];
                let dot_nv = sx * view[0] + sy * view[1] + sz * view[2];
                let mut rx = 2.0 * dot_nv * sx - view[0];
                let mut ry = 2.0 * dot_nv * sy - view[1];
                let mut rz = 2.0 * dot_nv * sz - view[2];
                let len = (rx * rx + ry * ry + rz * rz).sqrt();
                if len > 0.0001 {
                    rx /= len;
                    ry /= len;
                    rz /= len;
                }
                let u_env = 0.5 + rx.atan2(rz) / (2.0 * std::f32::consts::PI);
                let v_env = 0.5 - ry.asin() / std::f32::consts::PI;
                let env_x =
                    (u_env.fract() * sphere_w as f32).clamp(0.0, sphere_w as f32 - 1.0) as usize;
                let env_y =
                    (v_env.fract() * sphere_h as f32).clamp(0.0, sphere_h as f32 - 1.0) as usize;
                let env_idx = ((env_y * sphere_w as usize + env_x) * 4) as usize;
                let env_b = sphere_pixels[env_idx + 0] as f32;
                let env_g = sphere_pixels[env_idx + 1] as f32;
                let env_r = sphere_pixels[env_idx + 2] as f32;

                let env_blend = 0.5 * ((sr + sg + sb) / (3.0 * 255.0));
                out_r = (out_r * (1.0 - env_blend) + env_r * env_blend).min(255.0);
                out_g = (out_g * (1.0 - env_blend) + env_g * env_blend).min(255.0);
                out_b = (out_b * (1.0 - env_blend) + env_b * env_blend).min(255.0);

                pixels[offset + 0] = out_b as u8;
                pixels[offset + 1] = out_g as u8;
                pixels[offset + 2] = out_r as u8;
                pixels[offset + 3] = 255;
            }
        }

        let img = raster_from_data(
            &ImageInfo::new(
                (globe_size as i32, globe_size as i32),
                ColorType::BGRA8888,
                AlphaType::Unpremul,
                None,
            ),
            skia_safe::Data::new_copy(&pixels),
            globe_size * 4,
        )
        .unwrap();
        canvas.draw_image(&img, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: n_frames as u32,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_image!(
    "rotating_globe",
    rotating_globe,
    min_images = 1,
    max_images = 1
);
