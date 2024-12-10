use std::borrow::Cow;

use super::{Buffer, Error};

/// `Image` [`Block`] of a [`Content`].
pub trait Image: Buffer {
    /// Media type of the image.
    fn format(&self) -> image::ImageFormat;

    /// Base64 encoded string data (without any metadata).
    fn base64<'a>(&'a self) -> Cow<'a, str>;

    /// Convert into a html `img` tag with embedded base64 data.
    fn html(&self) -> String {
        format!(
            "<img src=\"data:image/{};base64,{}\" />",
            match self.format() {
                image::ImageFormat::Jpeg => "jpeg",
                image::ImageFormat::Png => "png",
                image::ImageFormat::Gif => "gif",
                image::ImageFormat::WebP => "webp",
                _ => "unknown",
            },
            self.base64()
        )
    }

    /// Convert into an [`image::RgbaImage`].
    fn into_image(self) -> Result<image::RgbaImage, Box<dyn Error>>;
}
static_assertions::assert_impl_all!(dyn Image: Buffer);
static_assertions::assert_obj_safe!(Image);
