use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::core::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ImageOption {
    Boolean {
        name: String,
        default: Option<bool>,
        description: Option<String>,
    },
    String {
        name: String,
        default: Option<String>,
        choices: Option<Vec<String>>,
        description: Option<String>,
    },
    Integer {
        name: String,
        default: Option<i32>,
        minimum: Option<i32>,
        maximum: Option<i32>,
        description: Option<String>,
    },
    Float {
        name: String,
        default: Option<f32>,
        minimum: Option<f32>,
        maximum: Option<f32>,
        description: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub min_images: u8,
    pub max_images: u8,
    pub min_texts: u8,
    pub max_texts: u8,
    pub default_texts: Vec<String>,
    pub options: Vec<ImageOption>,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            min_images: 0,
            max_images: 0,
            min_texts: 0,
            max_texts: 0,
            default_texts: Vec::new(),
            options: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub key: String,
    pub params: Params,
    pub keywords: Vec<String>,
    pub tags: HashSet<String>,
    pub date_created: DateTime<Local>,
    pub date_modified: DateTime<Local>,
}

impl Default for Info {
    fn default() -> Self {
        Info {
            key: String::new(),
            params: Params::default(),
            keywords: Vec::new(),
            tags: HashSet::new(),
            date_created: Local::now(),
            date_modified: Local::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionValue {
    Boolean(bool),
    String(String),
    Integer(i32),
    Float(f32),
}

impl Into<OptionValue> for bool {
    fn into(self) -> OptionValue {
        OptionValue::Boolean(self)
    }
}

impl Into<OptionValue> for String {
    fn into(self) -> OptionValue {
        OptionValue::String(self)
    }
}

impl Into<OptionValue> for &str {
    fn into(self) -> OptionValue {
        OptionValue::String(self.to_string())
    }
}

impl Into<OptionValue> for i32 {
    fn into(self) -> OptionValue {
        OptionValue::Integer(self)
    }
}

impl Into<OptionValue> for f32 {
    fn into(self) -> OptionValue {
        OptionValue::Float(self)
    }
}

pub trait ImageData: Send + Sync {
    fn key(&self) -> String;
    fn info(&self) -> Info;
    fn generate(
        &self,
        images: Vec<Image>,
        texts: Vec<String>,
        options: HashMap<String, OptionValue>,
    ) -> Result<Vec<u8>, Error>;
    fn generate_preview(&self, options: HashMap<String, OptionValue>) -> Result<Vec<u8>, Error>;
}

pub struct ImageDeclaration {
    pub name: &'static str,
    pub builder: fn() -> Box<dyn ImageData>,
}
