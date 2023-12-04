use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;


#[allow(dead_code)]
pub fn invert(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut output_img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let inverted_pixel = Rgba([
                255 - pixel[0],
                255 - pixel[1],
                255 - pixel[2],
                pixel[3],  // alpha channel is usually left unchanged
            ]);
            output_img.put_pixel(x, y, inverted_pixel);
        }
    }

    output_img
}


#[allow(dead_code)]
pub fn par_invert(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut output_img = img.clone().to_rgba8();

    output_img
        .par_chunks_mut(4)
        .for_each(|pixel| {
            pixel[0] = 255 - pixel[0];
            pixel[1] = 255 - pixel[1];
            pixel[2] = 255 - pixel[2];
        });

    output_img
}