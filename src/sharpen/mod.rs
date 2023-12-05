use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;


/*
The sharpening filter works by enhancing the contrast between a pixel and its neighboring pixels. 
The center pixel is strengthened (weight of 9) and the neighboring pixels are weakened (weight of -1). 
*/

#[allow(dead_code)]
pub fn sharpen(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    
    let mut output_img = ImageBuffer::new(width, height);

    for y in 1..height-1 {
        for x in 1..width-1 {
            let mut r = 0i32;
            let mut g = 0i32;
            let mut b = 0i32;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let pixel = img.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);
                    let weight = if dx == 0 && dy == 0 { 9 } else { -1 };  // Adjusted weights
                    r += pixel[0] as i32 * weight;
                    g += pixel[1] as i32 * weight;
                    b += pixel[2] as i32 * weight;
                }
            }

            let pixel = Rgba([
                r.max(0).min(255) as u8,
                g.max(0).min(255) as u8,
                b.max(0).min(255) as u8,
                img.get_pixel(x, y)[3],  // Preserve the original alpha channel
            ]);

            output_img.put_pixel(x, y, pixel);
        }
    }

    output_img
}

#[allow(dead_code)]
pub fn par_sharpen(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();

    let (lower_bound, upper_bound) = (-1, 1);

    // Initialize the array with the values of dx and dy
    let offsets: Vec<(i32, i32)> = (lower_bound..=upper_bound)
        .flat_map(|dx| (lower_bound..=upper_bound).map(move |dy| (dx, dy)))
        .collect();

    let output_img: Vec<_> = (0..height).into_par_iter().map(|y| {
        let mut row = vec![Rgba([0, 0, 0, 0]); width as usize];
        for x in 0..width {
            let mut r = 0i32;
            let mut g = 0i32;
            let mut b = 0i32;

            for &(dx, dy) in &offsets {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let pixel = img.get_pixel(nx as u32, ny as u32);
                    let weight = if dx == 0 && dy == 0 { 9 } else { -1 };
                    r += pixel[0] as i32 * weight;
                    g += pixel[1] as i32 * weight;
                    b += pixel[2] as i32 * weight;
                }
            }

            row[x as usize] = Rgba([
                r.max(0).min(255) as u8,
                g.max(0).min(255) as u8,
                b.max(0).min(255) as u8,
                img.get_pixel(x, y)[3],
            ]);
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
