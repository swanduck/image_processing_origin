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
    let (width, height) = img.dimensions();

    let output_img: Vec<_> = (0..height).into_par_iter().map(|y| {
        let mut row = vec![Rgba([0, 0, 0, 0]); width as usize];
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let inverted_pixel = Rgba([
                255 - pixel[0],
                255 - pixel[1],
                255 - pixel[2],
                pixel[3],  // alpha channel is usually left unchanged
            ]);
            row[x as usize] = inverted_pixel;
        }
        row
    }).collect();

    let mut output_img_buffer = ImageBuffer::new(width, height);
    for (y, row) in output_img.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            output_img_buffer.put_pixel(x as u32, y as u32, pixel);
        }
    }

    output_img_buffer
}
