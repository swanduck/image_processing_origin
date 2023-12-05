use image::{GenericImageView, DynamicImage, Pixel, ImageBuffer, Rgba};
use rayon::prelude::*;


// A value greater than 1.0 will increase contrast, while a value less than 1.0
//  (but greater than 0) will decrease contrast. A value of 1.0 will leave the contrast unchanged.
#[allow(dead_code)]
pub fn increase_contrast(img: &DynamicImage, contrast: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut output_img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let mut px = img.get_pixel(x, y);
            let channels = px.channels_mut();

            for channel in channels.iter_mut() {
                let color = *channel as f32 / 255.0; // Convert to float
                let color = (((color - 0.5) * contrast) + 0.5) * 255.0; // Apply contrast
                *channel = color.max(0.0).min(255.0) as u8; // Convert back to u8
            }

            output_img.put_pixel(x, y, px);
        }
    }

    output_img
}

#[allow(dead_code)]
pub fn par_contrast(img: &DynamicImage, contrast: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut output_img = ImageBuffer::new(width, height);

    output_img
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let mut px = img.get_pixel(x, y);
            let channels = px.channels_mut();

            for channel in channels.iter_mut() {
                let color = *channel as f32 / 255.0; // Convert to float
                let color = (((color - 0.5) * contrast) + 0.5) * 255.0; // Apply contrast
                *channel = color.max(0.0).min(255.0) as u8; // Convert back to u8
            }

            *pixel = px;
        });

    output_img
}

