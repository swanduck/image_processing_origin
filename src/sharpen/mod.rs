use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba, };

pub fn sharpen(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut output_img = ImageBuffer::new(width, height);

    for y in 1..height-1 {
        for x in 1..width-1 {
            let mut r = 0i32;
            let mut g = 0i32;
            let mut b = 0i32;
            let mut a = 0i32;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let pixel = img.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);
                    let weight = if dx == 0 && dy == 0 { 5 } else { -1 };
                    r += pixel[0] as i32 * weight;
                    g += pixel[1] as i32 * weight;
                    b += pixel[2] as i32 * weight;
                    a += pixel[3] as i32 * weight;
                }
            }

            let pixel = Rgba([
                r.max(0).min(255) as u8,
                g.max(0).min(255) as u8,
                b.max(0).min(255) as u8,
                a.max(0).min(255) as u8,
            ]);

            output_img.put_pixel(x, y, pixel);
        }
    }

    output_img
}
