use image_derive::ImageOptions;
use skia_safe::{Color, FontStyle, Image, Point, Rect, textlayout::TextAlign};

use crate::{
    core::error::Error,
    register_image, text_params,
    utils::{
        builder::InputImage,
        encoder::make_png_or_gif,
        text::Text2Image,
        tools::{color_from_hex_code, new_paint, new_surface},
    },
};

#[derive(ImageOptions)]
struct Font {
    #[option(default = "futura", choices = ["futura", "comic sans ms", "impact", "times", "roboto", "ubuntu", "helvetica", "arial"])]
    font: Option<String>,
}

fn caption(images: Vec<InputImage>, texts: Vec<String>, options: Font) -> Result<Vec<u8>, Error> {
    let mut font_families: [&'static str; 1] = [""];
    let font = match options.font.as_deref() {
        Some("comic sans ms") => Some("Comic Sans MS"),
        Some("times") => Some("Times New Roman"),
        Some("roboto") => Some("Roboto"),
        Some("impact") => Some("Impact"),
        Some("futura") => Some("Futura Extra Black Condensed"),
        Some("ubuntu") => Some("Ubuntu"),
        Some("helvetica") => Some("Helvetica Neue"),
        Some("arial") => Some("Arial"),
        _ => Some("Impact"),
    };

    if let Some(f) = font {
        font_families[0] = f;
    }

    let func = |images: Vec<Image>| {
        let img = images[0].clone();
        let img_w = img.width();
        let img_h = img.height();

        let text = &texts[0];
        let font_size = (img_w as f32 * 0.055).clamp(14.0, 80.0);
        let margin = (img_w as f32 * 0.03).clamp(8.0, 40.0);
        let wrap_width = (img_w as f32 - margin * 2.0).max(10.0);

        let t = Text2Image::from_text(
            text,
            font_size,
            text_params!(
                font_families = &font_families,
                text_align = TextAlign::Center,
                wrap_max_width = wrap_width,
                paint = new_paint(color_from_hex_code("#000000")),
                font_style = if font.unwrap_or_else(|| "") == "impact" {
                    FontStyle::normal()
                } else {
                    FontStyle::bold()
                },
            ),
        );
        let text_width = t.longest_line();
        let text_height = t.height();

        let caption_height = (text_height + margin * 2.0).ceil() as i32;

        let w = img_w;
        let h = img_h + caption_height;

        let mut surface = new_surface((w, h));
        let canvas = surface.canvas();

        let rect = Rect::from_xywh(0.0, 0.0, w as f32, caption_height as f32);
        let mut paint = new_paint(Color::WHITE);
        paint.set_anti_alias(true);
        canvas.draw_rect(rect, &paint);

        let text_x = ((w as f32) - text_width) / 2.0;
        let text_y = margin;
        t.draw_on_canvas(canvas, Point::new(text_x, text_y));

        canvas.draw_image(img, (0, caption_height), None);

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_image!(
    "caption",
    caption,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["A Caption"]
);
