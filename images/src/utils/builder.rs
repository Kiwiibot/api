use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::{Number, Value};

use skia_safe::{Codec, Data, Image};

use crate::core::types::{ImageData, Info, OptionValue, Params};
use crate::core::{error::Error, types, types::ImageOption};
use crate::utils::{decoder::CodecExtensions, encoder::encode_png, tools::grid_pattern_image};

pub use image_derive::ImageOptions;

pub trait ImageOptions: Default + for<'de> Deserialize<'de> + Send + Sync {
    fn to_options(&self) -> Vec<ImageOption>;
}

pub struct InputImage<'a> {
    pub name: String,
    pub image: Image,
    pub codec: Codec<'a>,
}

impl<'a> InputImage<'a> {
    pub fn from(input: &types::Image) -> Result<InputImage<'static>, Error> {
        let data = Data::new_copy(&input.data);
        let mut codec = Codec::from_data(data)
            .ok_or(Error::ImageDecodeError("Skia decode error".to_string()))?;
        let image = codec.first_frame()?;
        Ok(InputImage {
            name: input.name.clone(),
            image,
            codec,
        })
    }
}

type ImageFunction<T> = fn(Vec<InputImage>, Vec<String>, T) -> Result<Vec<u8>, Error>;

pub struct ImageBuilder<T>
where
    T: ImageOptions,
{
    pub key: String,
    pub min_images: u8,
    pub max_images: u8,
    pub min_texts: u8,
    pub max_texts: u8,
    pub default_texts: Vec<String>,
    pub options: T,
    pub keywords: Vec<String>,
    pub tags: HashSet<String>,
    pub date_created: DateTime<Local>,
    pub date_modified: DateTime<Local>,
    pub function: ImageFunction<T>,
}

impl<T> Default for ImageBuilder<T>
where
    T: ImageOptions,
{
    fn default() -> Self {
        ImageBuilder {
            key: String::new(),
            min_images: 0,
            max_images: 0,
            min_texts: 0,
            max_texts: 0,
            default_texts: Vec::new(),
            options: T::default(),
            keywords: Vec::new(),
            tags: HashSet::new(),
            date_created: Local::now(),
            date_modified: Local::now(),
            function: |_, _, _| Ok(Vec::new()),
        }
    }
}

pub mod image_setters {
    use chrono::{DateTime, Local};
    use std::collections::HashSet;

    pub fn min_images(min_images: u8) -> u8 {
        min_images
    }

    pub fn max_images(max_images: u8) -> u8 {
        max_images
    }

    pub fn min_texts(min_texts: u8) -> u8 {
        min_texts
    }

    pub fn max_texts(max_texts: u8) -> u8 {
        max_texts
    }

    pub fn default_texts(default_texts: &[&str]) -> Vec<String> {
        default_texts.iter().map(|text| text.to_string()).collect()
    }

    pub fn keywords(keywords: &[&str]) -> Vec<String> {
        keywords.iter().map(|keyword| keyword.to_string()).collect()
    }

    pub fn tags(tags: HashSet<String>) -> HashSet<String> {
        tags
    }

    pub fn date_created(date_created: DateTime<Local>) -> DateTime<Local> {
        date_created
    }

    pub fn date_modified(date_modified: DateTime<Local>) -> DateTime<Local> {
        date_modified
    }
}

impl<T> ImageData for ImageBuilder<T>
where
    T: ImageOptions,
{
    fn key(&self) -> String {
        self.key.clone()
    }

    fn info(&self) -> Info {
        Info {
            key: self.key.clone(),
            params: Params {
                min_images: self.min_images,
                max_images: self.max_images,
                min_texts: self.min_texts,
                max_texts: self.max_texts,
                default_texts: self.default_texts.clone(),
                options: self.options.to_options(),
            },
            keywords: self.keywords.clone(),
            tags: self.tags.clone(),
            date_created: self.date_created.clone(),
            date_modified: self.date_modified.clone(),
        }
    }

    fn generate(
        &self,
        images: Vec<types::Image>,
        texts: Vec<String>,
        options: HashMap<String, OptionValue>,
    ) -> Result<Vec<u8>, Error> {
        let info = self.info();
        if images.len() < info.params.min_images as usize
            || images.len() > info.params.max_images as usize
        {
            return Err(Error::ImageNumberMismatch(
                info.params.min_images,
                info.params.max_images,
                images.len() as u8,
            ));
        }
        if texts.len() < info.params.min_texts as usize
            || texts.len() > info.params.max_texts as usize
        {
            return Err(Error::TextNumberMismatch(
                info.params.min_texts,
                info.params.max_texts,
                texts.len() as u8,
            ));
        }

        let options = options
            .iter()
            .map(|(key, value)| {
                let value = match value {
                    OptionValue::Boolean(value) => Value::Bool(*value),
                    OptionValue::String(value) => Value::String(value.clone()),
                    OptionValue::Integer(value) => Value::Number(Number::from(*value)),
                    OptionValue::Float(value) => {
                        Value::Number(Number::from_f64(f64::from(*value)).unwrap())
                    }
                };
                (key.clone(), value)
            })
            .collect();

        let options = serde_json::from_value(Value::Object(options)).map_err(|err| {
            let e = err.to_string();
            let mut split = e.split(": ");
            let rest = split.nth(0);
            let value = split.last();
            if value.is_none() {
                return Error::DeserializeError(e);
            }
            let name = rest.unwrap_or_else(|| "").split(" ").last();

            if name.is_none() {
                return Error::DeserializeError(e);
            }

            let opt = info.params.options.iter().find(|e| match e {
                ImageOption::String {
                    name: n,
                    default: _,
                    choices: _,
                    description: _,
                } => n == name.unwrap(),
                _ => false,
            });
            if let Some(ImageOption::String {
                name,
                default: _,
                choices,
                description: _,
            }) = opt
            {
                if choices.is_some() {
                    Error::InvalidChoice(name.to_string(), choices.clone().unwrap(), value.unwrap().to_string())
                } else {
                    Error::DeserializeError(e)
                }
            } else {
                Error::DeserializeError(e)
            }
        })?;
        let images = images
            .iter()
            .map(|image| InputImage::from(image))
            .collect::<Result<Vec<InputImage>, Error>>()?;
        (self.function)(images, texts, options)
    }

    fn generate_preview(&self, options: HashMap<String, OptionValue>) -> Result<Vec<u8>, Error> {
        let mut images = Vec::new();
        if self.min_images > 0 {
            let image = encode_png(grid_pattern_image())?;
            for i in 0..self.min_images {
                let name = if self.min_images == 1 {
                    "{name}".to_string()
                } else {
                    format!("{{name{}}}", i + 1)
                };
                images.push(types::Image {
                    name: name,
                    data: image.clone(),
                });
            }
        }
        let texts = if self.default_texts.len() >= self.min_texts as usize
            && self.default_texts.len() <= self.max_texts as usize
        {
            self.default_texts.clone()
        } else {
            let mut texts = Vec::new();
            for i in 0..self.min_texts {
                let text = if self.min_texts == 1 {
                    "{text}".to_string()
                } else {
                    format!("{{text{}}}", i + 1)
                };
                texts.push(text);
            }
            texts
        };
        self.generate(images, texts, options)
    }
}
