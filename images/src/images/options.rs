use crate::utils::builder::ImageOptions;

#[derive(ImageOptions)]
pub struct NoOptions {}

#[derive(ImageOptions)]
pub(crate) struct Circle {
    /// Whether to transform the image in a circle.
    #[option(default = false)]
    pub circle: Option<bool>,
}
