mod pixelate;
mod blur;
mod invert;


use image::io::Reader as ImageReader;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let img = ImageReader::open("wallpaper.png")?.decode()?;
    use std::time::Instant;
    // use pixelate::*;
    // let img_ = pixelate(&img, (20, 20));
    // let _ = img_.save("images/pixel/pixelated.png");

    /*
    blur testing --------------------------------------------------
     */

    // use blur::*;
    // let blur_param = 30;
    // let now = Instant::now();
    // {
    //     let img_ = blur(&img, blur_param);
    //     let _ = img_.save("images/blur/blurred.png");    
    // }
    // let elapsed = now.elapsed();

    // let now2 = Instant::now();
    // {
    //     let img_ = par_blur(&img, blur_param);
    //     // let _ = img_.save("images/blur/par_blurred.png");    
    //     match img_.save("images/blur/par_blurred.png") {
    //         Ok(_) => println!("Image saved successfully."),
    //         Err(e) => println!("Failed to save image: {}", e),
    //     }
    // }
    // let elapsed2 = now2.elapsed();

    // println!("blur_seq: {:.2?}", elapsed);
    // println!("blur_par: {:.2?}", elapsed2);
    
    // let speedup = elapsed.as_secs_f64() / elapsed2.as_secs_f64();
    // println!("blur_par is {:.2} times faster than the sequential version.", speedup);

    /*
    invert testing --------------------------------------------------
     */

    use invert::*;

    let now_invert = Instant::now();
    {
        let img_ = invert(&img);
        match img_.save("images/invert/inverted.png") {
            Ok(_) => println!("Image saved successfully."),
            Err(e) => println!("Failed to save image: {}", e),
        }
    }
    let elapsed_invert = now_invert.elapsed();



    let now_parinvert = Instant::now();
    {
        let img_ = par_invert(&img);
        match img_.save("images/invert/par_inverted.png") {
            Ok(_) => println!("Image saved successfully."),
            Err(e) => println!("Failed to save image: {}", e),
        }
    }
    let elapsed_parinvert = now_parinvert.elapsed();
 
    println!("invert_seq: {:.2?}", elapsed_invert);
    println!("invert_par: {:.2?}", elapsed_parinvert);
    let invert_speedup = elapsed_invert.as_secs_f64() / elapsed_parinvert.as_secs_f64();
    println!("par_invert is {:.2} times faster than the sequential version.", invert_speedup);




    Ok(())
}

