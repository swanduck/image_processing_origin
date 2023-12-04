use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;


//par = blurring parameters, par = 3 meaning 3x3 neighbor squares
#[allow(dead_code)]
pub fn blur(img: &DynamicImage, par: i32) -> Image {
    let (width, height) = img.dimensions();
    let mut output_img = ImageBuffer::new(width, height);

    let bound: i32;
    let squared: u32;
    // if even par, use the next odd number (par + 1), eg. 4 --> 5
    if par % 2 == 0 {
        bound = par/2;
        squared = ((par+1) * (par+1)) as u32;
    }
    else {
        bound = (par-1)/2;
        squared = (par * par) as u32;
    }
    let (lower_bound, upper_bound) = (bound * -1, bound);

    // println!("{lower_bound}, {upper_bound}, {squared}");

    for y in 0..height {
        for x in 0..width {
            let mut r = 0u32;
            let mut g = 0u32;
            let mut b = 0u32;
            let mut a = 0u32;

            for dy in lower_bound..=upper_bound {
                for dx in lower_bound..=upper_bound {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        let pixel = img.get_pixel(nx as u32, ny as u32);
                        r += pixel[0] as u32;
                        g += pixel[1] as u32;
                        b += pixel[2] as u32;
                        a += pixel[3] as u32;
                        //adding neighbor colors to the totals 
                    }
                }
            }

            let pixel = Rgba([
                (r / squared) as u8,
                (g / squared) as u8,
                (b / squared) as u8,
                (a / squared) as u8,
            ]);
            //getting the average 

            output_img.put_pixel(x, y, pixel);
        }
    }

    output_img
}

#[allow(dead_code)]
pub fn par_blur(img: &DynamicImage, par: i32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    
    let bound: i32;
    let squared: u32;
    if par % 2 == 0 {
        bound = par/2;
        squared = ((par+1) * (par+1)) as u32;
    }
    else {
        bound = (par-1)/2;
        squared = (par * par) as u32;
    }
    let (lower_bound, upper_bound) = (bound * -1, bound);

    // Initialize the array with the values of dx and dy
    let offsets: Vec<(i32, i32)> = (lower_bound..=upper_bound)
        .flat_map(|dx| (lower_bound..=upper_bound).map(move |dy| (dx, dy)))
        .collect();

    let output_img: Vec<_> = (0..height).into_par_iter().map(|y| {
        let mut row = vec![Rgba([0, 0, 0, 0]); width as usize];
        for x in 0..width {
            let mut r = 0u32;
            let mut g = 0u32;
            let mut b = 0u32;
            let mut a = 0u32;

            for &(dx, dy) in &offsets {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let pixel = img.get_pixel(nx as u32, ny as u32);
                    r += pixel[0] as u32;
                    g += pixel[1] as u32;
                    b += pixel[2] as u32;
                    a += pixel[3] as u32;
                }
            }

            row[x as usize] = Rgba([
                (r / squared) as u8,
                (g / squared) as u8,
                (b / squared) as u8,
                (a / squared) as u8,
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
