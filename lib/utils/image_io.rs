use image::{GenericImageView, Pixel, Rgba};

use crate::core::{image::Images, pixel::Pixels};

pub fn image_reader(filepath: &str) -> Result<Images, Box<dyn std::error::Error>> {
    let read_image = image::ImageReader::open(filepath)?.decode()?;

    println!("Starting to read image from {:?} . . .", filepath);
    let mut image_bytes: Vec<Pixels> = Vec::new();
    for h_index in 0..read_image.height() {
        // read column then row
        for w_index in 0..read_image.width() {
            let pixel = read_image.get_pixel(w_index, h_index).to_rgba();
            let r = pixel.channels().get(0);
            let g = pixel.channels().get(1);
            let b = pixel.channels().get(2);
            let a = pixel.channels().get(3);
            image_bytes.push(Pixels::new(
                *r.unwrap(),
                *g.unwrap(),
                *b.unwrap(),
                *a.unwrap(),
            ));
        }
    }

    let image = Images::new(
        read_image.width(),
        read_image.height(),
        read_image.color().channel_count(),
        image_bytes,
    );
    println!("Read image from {:?}", filepath);

    Ok(image)
}

pub fn image_writer(
    filepath: &str,
    write_image: &Images,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut image: image::DynamicImage =
        image::DynamicImage::new_rgba8(write_image.get_width(), write_image.get_height());

    println!("Starting to write image to {:?} . . .", filepath);
    let rgba_image = image.as_mut_rgba8().unwrap();
    for (index, pixel) in rgba_image.pixels_mut().enumerate() {
        let rgba = &write_image;
        *pixel = Rgba([
            rgba.get_image().get(index).unwrap().get_red(),
            rgba.get_image().get(index).unwrap().get_green(),
            rgba.get_image().get(index).unwrap().get_blue(),
            rgba.get_image().get(index).unwrap().get_alpha(),
        ]);
    }

    image.save(filepath)?;
    println!("Saved image to {:?}", filepath);

    Ok(())
}
