use image::{ImageResult, DynamicImage,  GenericImageView};

fn main() -> ImageResult<()>{
    let text = image_to_text("imgs/Thumbsup1.png", 60, 60, 60.0)?;
    println!("{}",text);
    Ok(())
}

fn load_image(path:&str) -> ImageResult<DynamicImage>{
    image::open(path)
}

fn image_to_text(path: &str, width: u32, height: u32, threshold: f64) -> ImageResult<String> {
    let img = load_image(path)?;
    let resized_img = img.resize(width, height, image::imageops::FilterType::Gaussian);
    let mut t : String = "".to_string();
    for y in 0..resized_img.height() {
        for x in 0..resized_img.width() {
            let pixel = resized_img.get_pixel(x, y);
            let glay: f64 = (pixel.0[0] as f64) * 0.3 + (pixel.0[1]  as f64) * 0.59 + (pixel.0[2] as f64) * 0.11;
            if glay > threshold {
                t += "#";
            }else{
                t += " ";
            }
        }
        t += "\n";
    }
    Ok(t)
}


#[cfg(test)]
mod tests {
    use crate::{load_image, image_to_text};

    #[test]
    fn load_image_test() {
       let img = load_image("imgs/Thumbsup1.png");
       match img {
        Ok(img) => {
            assert_eq!(img.width(),1113);
            assert_eq!(img.height(), 1400);
        },
        Err(e) => {
            panic!("{}",e);
        }
       }
    }

    #[test]
    fn image_to_text_test(){
        let text = image_to_text("imgs/Thumbsup1.png", 60, 60, 125.0);
        match text {
            Ok(text) => {
                assert!(true);
            },
            Err(e) => {
                panic!("{}",e);
            }
        }
    }
}
