use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};


#[allow(dead_code)]
pub fn invert_colors(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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