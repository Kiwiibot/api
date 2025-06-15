use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    ImageDecodeError(String),
    ImageEncodeError(String),
    ImageAssetMissing(String),
    DeserializeError(String),
    ImageNumberMismatch(u8, u8, u8),
    TextNumberMismatch(u8, u8, u8),
    InvalidChoice(String, Vec<String>, String),
    TextOverLength(String),
    Generic(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageDecodeError(err) => write!(f, "Failed to decode image: {err}"),
            Self::ImageEncodeError(err) => write!(f, "Failed to encode image: {err}"),
            Self::ImageAssetMissing(err) => write!(f, "Image asset missing: {err}"),
            Self::DeserializeError(err) => write!(f, "Failed to deserialize: {err}"),
            Self::ImageNumberMismatch(min, max, actual) => write!(
                f,
                "Image number mismatch, got {actual} but expected between {min} and {max}"
            ),
            Self::TextNumberMismatch(min, max, actual) => write!(
                f,
                "Text number mismatch, got {actual} but expected between {min} and {max}"
            ),
            Self::TextOverLength(text) => write!(f, "Text is too long: {text}"),
            Self::InvalidChoice(name, choices, text) => write!(f, "Invalid choice for {name}: given {text}, expected one of [{}]", choices.join(", ")),
            Self::Generic(err) => write!(f, "{err}"),
        }
    }
}

impl error::Error for Error {}
