use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;

pub fn grayscale_ntsc(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut output_img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let gray = (pixel[0] as f32 * 0.299 + pixel[1] as f32 * 0.587 + pixel[2] as f32 * 0.114) as u8;
            let grayscale_pixel = Rgba([gray, gray, gray, pixel[3]]);
            output_img.put_pixel(x, y, grayscale_pixel);
        }
    }

    output_img
}


// pub fn par_grayscale(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
//     let (width, height) = img.dimensions();

//     let output_img: Vec<_> = (0..height).into_par_iter().map(|y| {
//         let mut row = vec![Rgba([0, 0, 0, 0]); width as usize];
//         for x in 0..width {
//             let pixel = img.get_pixel(x, y);
//             let gray = (pixel[0] as f32 * 0.299 + pixel[1] as f32 * 0.587 + pixel[2] as f32 * 0.114) as u8;
//             let grayscale_pixel = Rgba([gray, gray, gray, pixel[3]]);
//             row[x as usize] = grayscale_pixel;
//         }
//         row
//     }).collect();

//     let mut output_img_buffer = ImageBuffer::new(width, height);
//     for (y, row) in output_img.iter().enumerate() {
//         for (x, &pixel) in row.iter().enumerate() {
//             output_img_buffer.put_pixel(x as u32, y as u32, pixel);
//         }
//     }

//     output_img_buffer
// }




pub fn par_grayscale(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut output_img = img.clone().to_rgba8();

    output_img
        .par_chunks_mut(4)
        .for_each(|pixel| {
            let gray = (pixel[0] as f32 * 0.299 + pixel[1] as f32 * 0.587 + pixel[2] as f32 * 0.114) as u8;
            pixel[0] = gray;
            pixel[1] = gray;
            pixel[2] = gray;
        });

    output_img
}


