use image::io::Reader as ImageReader;
use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[allow(dead_code)]
fn resize(img: &Image, new_dims: (u32, u32)) -> Image {
    let (old_width, old_height) = img.dimensions();
    let (new_width, new_height) = new_dims;

    let mut resized = ImageBuffer::new(new_width, new_height);

    for (new_x, new_y, pixel) in resized.enumerate_pixels_mut() {
        let old_x = (new_x as f32 * (old_width  as f32 / new_width  as f32)) as u32;
        let old_y = (new_y as f32 * (old_height  as f32 / new_height  as f32)) as u32;

        if let Some(old_pixel) = img.get_pixel_checked(old_x, old_y) {
            *pixel = *old_pixel;
        } else {
            println!("({old_x} -> {new_x}, {old_y} -> {new_y})");
        }
    }

    resized
}

#[allow(dead_code)]
fn pixelate(img: &DynamicImage, new_dims: (u32, u32)) -> Image {
    let old_dims = img.dimensions();

    let img = img.to_rgba8();

    let small = resize(&img, ((old_dims.0/new_dims.0), (old_dims.1/new_dims.1)));
    small.save("debug-small.png").unwrap();

    let pixelated = resize(&small, old_dims);
    pixelated.save("debug-pixelated.png").unwrap();
    pixelated

}

//par = blurring parameters, par = 3 meaning 3x3 neighbor squares
fn blur(img: &DynamicImage, par: i32) -> Image {
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

fn par_blur(img: &DynamicImage, par: i32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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

    let output_img: Vec<_> = (0..height).into_par_iter().map(|y| {
        let mut row = vec![Rgba([0, 0, 0, 0]); width as usize];
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
                    }
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





fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let img = ImageReader::open("burger.png")?.decode()?;
    // let img_ = pixelate(&img, (20, 20));
    // let _ = img_.save("images/pixel/pixelated.png");


    use std::time::Instant;
    let now = Instant::now();
    {
        let img_ = blur(&img, 40);
        let _ = img_.save("images/blur/blurred.png");    
    }
    let elapsed = now.elapsed();


    let now2 = Instant::now();
    {
        let img_ = par_blur(&img, 40);
        // let _ = img_.save("images/blur/par_blurred.png");    
        match img_.save("images/blur/par_blurred.png") {
            Ok(_) => println!("Image saved successfully."),
            Err(e) => println!("Failed to save image: {}", e),
        }
        
    }
    let elapsed2 = now2.elapsed();
    println!("seq: {:.2?}", elapsed);
    println!("par: {:.2?}", elapsed2);
    
    let speedup = elapsed.as_secs_f64() / elapsed2.as_secs_f64();
    println!("The parallel version is {:.2} times faster than the sequential version.", speedup);

    Ok(())
}

