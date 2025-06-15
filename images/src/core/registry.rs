use std::{collections::HashMap, sync::LazyLock};

use crate::core::types::{ImageData, ImageDeclaration};

inventory::collect!(ImageDeclaration);

#[macro_export]
macro_rules! register_image {
    ($key:expr, $function:expr, $($field:ident = $value:expr),* $(,)?) => {
        inventory::submit! {
            $crate::core::types::ImageDeclaration {
                name: $key,
                builder: || -> Box<dyn $crate::core::types::ImageData> {
                    Box::new(
                        $crate::utils::builder::ImageBuilder {
                            key: $key.to_string(),
                            function: $function,
                            $(
                                $field: $crate::utils::builder::image_setters::$field($value),
                            )*
                            ..Default::default()
                        }
                    )
                }
            }
        }
    }
}

pub fn load_images() -> HashMap<String, Box<dyn ImageData>> {
    let mut images: HashMap<String, Box<dyn ImageData>> = HashMap::new();

    for image_decl in inventory::iter::<ImageDeclaration> {
        images.insert(image_decl.name.to_string(), (image_decl.builder)());
    }

    images
}

pub static LOADED_IMAGES: LazyLock<HashMap<String, Box<dyn ImageData>>> =
    LazyLock::new(|| load_images());

pub fn get_image(key: &str) -> Option<&'static Box<dyn ImageData>> {
    LOADED_IMAGES.get(key)
}
