mod pixelate;
mod blur;
mod invert;
mod sharpen;
mod grayscale;
mod contrast;


use image::io::Reader as ImageReader;
use std::time::Instant;
use image::{DynamicImage, ImageBuffer, Rgba};


fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let img = ImageReader::open("wallpaper.png")?.decode()?;
    use std::time::Instant;


//     // use pixelate::*;
//     // let img_ = pixelate(&img, (20, 20));
//     // let _ = img_.save("images/pixel/pixelated.png");

//     /*
//     blur testing --------------------------------------------------
//      */

//     // use blur::*;
//     // let blur_param = 30;
//     // let now = Instant::now();
//     // {
//     //     let img_ = blur(&img, blur_param);
//     //     let _ = img_.save("images/blur/blurred.png");    
//     // }
//     // let elapsed = now.elapsed();

//     // let now2 = Instant::now();
//     // {
//     //     let img_ = par_blur(&img, blur_param);
//     //     // let _ = img_.save("images/blur/par_blurred.png");    
//     //     match img_.save("images/blur/par_blurred.png") {
//     //         Ok(_) => println!("Image saved successfully."),
//     //         Err(e) => println!("Failed to save image: {}", e),
//     //     }
//     // }
//     // let elapsed2 = now2.elapsed();

//     // println!("blur_seq: {:.2?}", elapsed);
//     // println!("blur_par: {:.2?}", elapsed2);
    
//     // let speedup = elapsed.as_secs_f64() / elapsed2.as_secs_f64();
//     // println!("blur_par is {:.2} times faster than the sequential version.", speedup);

//     /*
//     invert testing --------------------------------------------------
//      */

//     // use invert::*;

//     // let now_invert = Instant::now();
//     // {
//     //     let img_ = invert(&img);
//     //     match img_.save("images/invert/inverted.png") {
//     //         Ok(_) => println!("Image saved successfully."),
//     //         Err(e) => println!("Failed to save image: {}", e),
//     //     }
//     // }
//     // let elapsed_invert = now_invert.elapsed();



//     // let now_parinvert = Instant::now();
//     // {
//     //     let img_ = par_invert(&img);
//     //     match img_.save("images/invert/par_inverted.png") {
//     //         Ok(_) => println!("Image saved successfully."),
//     //         Err(e) => println!("Failed to save image: {}", e),
//     //     }
//     // }
//     // let elapsed_parinvert = now_parinvert.elapsed();
 
//     // println!("invert_seq: {:.2?}", elapsed_invert);
//     // println!("invert_par: {:.2?}", elapsed_parinvert);
//     // let invert_speedup = elapsed_invert.as_secs_f64() / elapsed_parinvert.as_secs_f64();
//     // println!("par_invert is {:.2} times faster than the sequential version.", invert_speedup);


    /*
    sharpen testing --------------------------------------------------
     */

    // use sharpen::*;
    // let now_sharpen = Instant::now();
    // {
    //     let img_ = sharpen(&img);
    //     match img_.save("images/sharpen/sharpened.png") {
    //         Ok(_) => println!("Image saved successfully."),
    //         Err(e) => println!("Failed to save image: {}", e),
    //     }
    // }
    // let elapsed_sharpen = now_sharpen.elapsed();
    
    //     let now_parsharp = Instant::now();
    // {
    //     let img_ = par_sharpen(&img);
    //     match img_.save("images/sharpen/par_sharpened.png") {
    //         Ok(_) => println!("Image saved successfully."),
    //         Err(e) => println!("Failed to save image: {}", e),
    //     }
    // }
    // let elapsed_parsharp = now_parsharp.elapsed();
    // let sharpen_speedup = elapsed_sharpen.as_secs_f64()/elapsed_parsharp.as_secs_f64();
    
    // println!("sharpen_seq: {:.2?}", elapsed_sharpen);
    // println!("sharpen_par: {:.2?}", elapsed_parsharp);
    // println!("sharpen_par is {:.2} times faster than the sequential version.", sharpen_speedup);








//     /*
//     grayscale testing --------------------------------------------------
//      */

    // use grayscale::*;
    // let now_gray = Instant::now();
    // {
    //     let img_ = grayscale_ntsc(&img);
    //     match img_.save("images/grayscale/grayscaled.png") {
    //         Ok(_) => println!("Image saved successfully."),
    //         Err(e) => println!("Failed to save image: {}", e),
    //     }
    // }
    // let elapsed_gray = now_gray.elapsed();

    // let now_pargray = Instant::now();
    // {
    //     let img_ = par_grayscale(&img);
    //     match img_.save("images/grayscale/par_grayscaled.png") {
    //         Ok(_) => println!("Image saved successfully."),
    //         Err(e) => println!("Failed to save image: {}", e),
    //     }
    // }
    // let elapsed_pargray = now_pargray.elapsed();

    // println!("grayscale_seq: {:.2?}", elapsed_gray);
    // println!("grayscale_par: {:.2?}", elapsed_pargray);


    /*
    contrast testing --------------------------------------------------
     */

    use contrast::*;

    process_and_save_image(&img, |img| increase_contrast(&img, 2.0), "images/contrast/contrasted.png");


    // let now_contrast = Instant::now();
    // {
    //     let img_ = increase_contrast(&img,2.0);
    //     match img_.save("images/contrast/contrasted.png") {
    //         Ok(_) => println!("Image saved successfully."),
    //         Err(e) => println!("Failed to save image: {}", e),
    //     }
    // }
    // let elapsed_contrast = now_contrast.elapsed();

    // let now_pcontrast = Instant::now();
    // {
    //     let img_ = par_contrast(&img,2.0);
    //     match img_.save("images/contrast/par_contrasted.png") {
    //         Ok(_) => println!("Image saved successfully."),
    //         Err(e) => println!("Failed to save image: {}", e),
    //     }
    // }
    // let elapsed_pcontrast = now_pcontrast.elapsed();

    // println!("contrast_seq: {:.2?}", elapsed_contrast);
    // println!("contrast_par: {:.2?}", elapsed_pcontrast);

    Ok(())
}



fn process_and_save_image<F>(img: &DynamicImage, operation: F, save_location: &str) 
where
    F: Fn(&DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>>,
{
    let now = Instant::now();
    {
        let img_ = operation(img);
        match img_.save(save_location) {
            Ok(_) => println!("Image saved successfully."),
            Err(e) => println!("Failed to save image: {}", e),
        }
    }
    let elapsed = now.elapsed();
    println!("Time elapsed in processing and saving the image is: {:?}", elapsed);
}
