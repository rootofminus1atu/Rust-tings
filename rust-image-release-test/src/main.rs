use std::{env, num::NonZeroU32, time::{Duration, Instant}};
use tokio::io::AsyncReadExt;
use tracing::{info, debug};
use tracing_subscriber::{self, fmt::format};
use tokio::fs::File;
use image::{imageops, DynamicImage, GenericImageView, RgbaImage};
use fast_image_resize as fr;

async fn overlay_soyjaks(base_img: &DynamicImage, soy_right: &DynamicImage, soy_left: &DynamicImage) -> DynamicImage {
    let mut new_img = base_img.clone();

    imageops::overlay(&mut new_img, soy_left, 0, base_img.height() as i64 - soy_left.height() as i64);
    imageops::overlay(&mut new_img, soy_right, base_img.width() as i64 - soy_right.width() as i64, base_img.height() as i64 - soy_right.height() as i64);

    new_img
}

fn slow_resize_to_height(image: &DynamicImage, new_height: u32) -> DynamicImage {
    let (width, height) = image.dimensions();
    let new_width = (new_height * width) / height;
    image.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3)
}

fn fast_resize_to_width(image: &DynamicImage, new_width: u32) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
    let (width, height) = image.dimensions();
    let new_height = (new_width * height) / width;

    let new_image = fast_resize(
        image, 
        NonZeroU32::new(new_width).unwrap(), 
        NonZeroU32::new(new_height).unwrap()
    )?;

    Ok(new_image)
}

fn fast_resize_with_scale_factor(image: &DynamicImage, scale_factor: f64) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
    let new_w = (image.width() as f64 * scale_factor).round() as u32;
    let new_h = (image.height() as f64 * scale_factor).round() as u32;

    let new_image = fast_resize(
        image, 
        NonZeroU32::new(new_w).unwrap(), 
        NonZeroU32::new(new_h).unwrap()
    )?;

    Ok(new_image)
}


fn fast_resize(image: &DynamicImage, new_w: NonZeroU32, new_h: NonZeroU32) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
    let (w, h) = image.dimensions();
    let mut src_img = fr::Image::from_vec_u8(
        NonZeroU32::new(w).unwrap(),
        NonZeroU32::new(h).unwrap(),
        image.clone().into_rgba8().into_raw(),
        fast_image_resize::PixelType::U8x4,
    )?;

    // Multiple RGB channels of source image by alpha channel 
    // (not required for the Nearest algorithm)
    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div.multiply_alpha_inplace(&mut src_img.view_mut())?;

    // Create container for data of destination image
    let mut dst_image = fr::Image::new(
        new_w,
        new_h,
        src_img.pixel_type(),
    );

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = fr::Resizer::new(
        fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3),
    );
    resizer.resize(&src_img.view(), &mut dst_view)?;

    // Divide RGB channels of destination image by alpha
    alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

    // save the new image
    let new_img = RgbaImage::from_raw(
        new_w.get(), 
        new_h.get(), 
        dst_image.into_vec()
    )
    .ok_or("Could not create image from raw u8 bytes after resizing")?;
    
    // old method, not sure if it's for sure slower but that's what it seemed like
    /* 
    let mut result_buf = BufWriter::new(Vec::new());
    PngEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            new_w.get(),
            new_h.get(),
            ColorType::Rgba8.into(),
        )
        .unwrap();

    let new_img = image::load_from_memory(&result_buf.into_inner().unwrap())?;
    */
    
    Ok(new_img.into())
}


#[derive(Debug, Clone)]
struct ImgDimF64 {
    w: f64,
    h: f64
}

impl ImgDimF64 {
    fn new(w: u32, h: u32) -> Self {
        Self { w: w as f64, h: h as f64 }
    }

    fn from_tuple(tuple: (u32, u32)) -> Self {
        Self::new(tuple.0, tuple.1)
    }

    fn from_img(img: &DynamicImage) -> Self {
        Self::from_tuple(img.dimensions())
    }

    fn rescale(&self, scale_factor: f64) -> Self {
        Self {
            w: self.w * scale_factor,
            h: self.h * scale_factor
        }
    }
}

/// TODO: Write tests for this
fn get_scale_factor(base_dim: ImgDimF64, soy_left_dim: ImgDimF64, soy_right_dim: ImgDimF64, min_w_gap_percentage: f64, min_h_gap_top_percentage: f64) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    // safety checks for the percentages
    if min_w_gap_percentage > 1.0 || min_w_gap_percentage < 0.0 {
        return Err(format!("{} is not a valid width gap percentage", min_w_gap_percentage).into());
    }

    if min_h_gap_top_percentage > 1.0 || min_h_gap_top_percentage < 0.0 {
        return Err(format!("{} is not a valid height gap percentage", min_h_gap_top_percentage).into());
    }

    debug!("base img size: {:?}", base_dim);

    // prearing the available widths and heights
    let min_w_gap = base_dim.w * min_w_gap_percentage;
    let available_w = base_dim.w - min_w_gap;

    let min_h_gap_top = base_dim.h * min_h_gap_top_percentage;
    let available_h = base_dim.h - min_h_gap_top;

    // first, we need to guarantee that there's gonna be enough of a width gap between the 2 soyjaks
    let scale_factor = available_w / (soy_left_dim.w + soy_right_dim.w); 

    // we create our rescale candidates
    let new_soy_left_dim = soy_left_dim.rescale(scale_factor);
    let new_soy_right_dim = soy_right_dim.rescale(scale_factor);

    debug!("soyjaks so far: {:?} - {:?}", new_soy_left_dim, new_soy_right_dim);

    // pick out the higher ORIGINAL soyjak img (although they SHOULD BE the same height)
    let higher_one = if soy_left_dim.h > soy_right_dim.h {
        soy_left_dim.clone()
    } else {
        soy_right_dim.clone()
    };

    
    // second, we check if the scale factor that we got will result in the height fitting under the required height gap
    let final_scale_factor = if new_soy_left_dim.h.max(new_soy_right_dim.h) < available_h {
        // if yes, we keep the scale factor as is
        scale_factor
    } else {
        // otherwise, we scale it down further, to match the required height gap
        let fixed_factor = available_h / higher_one.h;

        fixed_factor
    };

    // these aren't needed, they're here just for debugging
    let final_soy_left_dim = soy_left_dim.rescale(final_scale_factor);
    let final_soy_right_dim = soy_right_dim.rescale(final_scale_factor);

    debug!("soyjaks better: {:?} - {:?}", final_soy_left_dim, final_soy_right_dim);

    Ok(final_scale_factor)
}

use lazy_static::lazy_static;

lazy_static! {
    static ref SOY_LEFT: Result<image::DynamicImage, image::ImageError> = {
        image::open("assets/soy-left.png")
    };
}

lazy_static! {
    static ref SOY_RIGHT: Result<image::DynamicImage, image::ImageError> = {
        image::open("assets/soy-right.png")
    };
}

/// too slow lol
const SOY_RIGHT_B: &[u8] = include_bytes!("..\\assets\\soy-right.png");


/// The docs below are incomprehensible if you are hovering over the function name, so please check the source code instead.
/// 
/// =================================
/// |                               | <- min_h_gap_top (here the percentage is around 20%)
/// |-------------------------------|
/// |        |          |           |
/// |        |          |           |
/// |soy_left|          | soy_right |
/// |        |          |           |
/// |        |          |           |
/// =================================
///               ^
///               min-w-gap (here the percentage is around 33%)
async fn soyjakify(img: Vec<u8>, min_w_gap_percentage: f64, min_h_gap_top_percentage: f64) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    
    info!("time stats for soyjakifying");

    let rn = Instant::now();
    let base_image = image::load_from_memory(&img)?;
    info!("reading base image {:?}", rn.elapsed());

    let rn = Instant::now();
    let soy_left = image::open("assets/soy-left.png")?;
    info!("reading soy_left {:?}", rn.elapsed());

    let rn = Instant::now();
    let soy_right = image::open("assets/soy-right.png")?;
    info!("reading soy_right {:?}", rn.elapsed());

    
    let rn = Instant::now();
    let scale_factor = get_scale_factor(
        ImgDimF64::from_img(&base_image), 
        ImgDimF64::from_img(&soy_left), 
        ImgDimF64::from_img(&soy_right), 
        min_w_gap_percentage, 
        min_h_gap_top_percentage
    )?;
    info!("calculated scale factor {:?}", rn.elapsed());

    let rn = Instant::now();
    let soy_left = fast_resize_with_scale_factor(&soy_left, scale_factor)?;
    info!("resized soy_left {:?}", rn.elapsed());

    let rn = Instant::now();
    let soy_right = fast_resize_with_scale_factor(&soy_right, scale_factor)?;
    info!("resized soy_right {:?}", rn.elapsed());

    
    let rn = Instant::now();
    let new_image = overlay_soyjaks(&base_image, &soy_right, &soy_left).await;
    info!("overlaid soys {:?}", rn.elapsed());
    
    

    Ok(new_image.into_bytes())
}





#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // let sf = get_scale_factor(
    //     ImgDimF64::new(5000, 2000), 
    //     ImgDimF64::new(1000, 4000), 
    //     ImgDimF64::new(1500, 4000), 
    //     0.5, 
    //     0.2
    // )?;


    if let Ok(current_dir) = env::current_dir() {
        println!("Current directory: {}", current_dir.display());
    } else {
        eprintln!("Failed to get current directory");
    }


    let mut file = File::open("assets/test-img.jpg").await?;
    let mut img_bytes = Vec::new();
    file.read_to_end(&mut img_bytes).await?;

    info!("f: {}", img_bytes.len());

    let res = soyjakify(img_bytes.clone(), 0.3, 0.2).await?;

    tokio::time::sleep(Duration::from_secs(2)).await;


    let res = soyjakify(img_bytes, 0.3, 0.2).await?;

    

    Ok(())
}

