use serde::{Deserialize, Serialize};
use skia_safe::{Codec, Data};

use crate::{core::error::Error, utils::decoder::CodecExtensions};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub width: i32,
    pub height: i32,
    pub is_multi_frame: bool,
    pub frame_count: Option<i32>,
    pub average_duration: Option<f32>,
}

pub(super) fn decode_image(data: Vec<u8>) -> Result<Codec<'static>, Error> {
    let data = Data::new_copy(&data);
    Codec::from_data(data).ok_or(Error::ImageDecodeError("Skia decode error".to_string()))
}

pub fn inspect(image: Vec<u8>) -> Result<ImageInfo, Error> {
    let mut codec = decode_image(image)?;
    let is_multi_frame = codec.is_multi_frame();
    let frame_count = if is_multi_frame {
        Some(codec.get_frame_count() as i32)
    } else {
        None
    };
    let average_duration = if is_multi_frame {
        Some(codec.get_avg_duration()?)
    } else {
        None
    };
    Ok(ImageInfo {
        width: codec.dimensions().width,
        height: codec.dimensions().height,
        is_multi_frame,
        frame_count,
        average_duration,
    })
}
