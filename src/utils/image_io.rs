use image::{GenericImageView, Pixel, Rgba};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, pixel::Pixels};

pub fn image_reader(filepath: &str) -> Result<Images, Box<dyn std::error::Error>> {
    let read_image = image::ImageReader::open(filepath)?.decode()?;

    println!("INFO: Starting to read image from {:?} . . .", filepath);
    let mut image_bytes: Vec<Pixels> = Vec::new();

    let read_pixels = (0..read_image.height())
        .into_par_iter()
        .flat_map(|h_index| {
            (0..read_image.width())
                .into_par_iter()
                .map(|w_index| {
                    let pixel = read_image.get_pixel(w_index, h_index).to_rgba();
                    Pixels::new(
                        *pixel.channels().first().unwrap(),
                        *pixel.channels().get(1).unwrap(),
                        *pixel.channels().get(2).unwrap(),
                        *pixel.channels().get(3).unwrap(),
                    )
                })
                .collect::<Vec<Pixels>>()
        })
        .collect::<Vec<Pixels>>();

    for pix in read_pixels.iter() {
        image_bytes.push(pix.clone());
    }

    let image = Images::new(
        read_image.width(),
        read_image.height(),
        read_image.color().channel_count(),
        image_bytes,
    );
    println!("INFO: Finished reading image from {:?}", filepath);

    Ok(image)
}

pub fn image_writer(
    filepath: &str,
    write_image: &Images,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut image: image::DynamicImage =
        image::DynamicImage::new_rgba8(write_image.get_width(), write_image.get_height());

    println!("INFO: Starting to write image to {:?} . . .", filepath);
    let rgba_image = image.as_mut_rgba8().unwrap();
    rgba_image
        .enumerate_pixels_mut()
        .for_each(|(x_index, y_index, pixel)| {
            let rgba = &write_image;
            let pixel_data = rgba.get_pixel_at(x_index, y_index).unwrap();
            *pixel = Rgba([
                pixel_data.get_red(),
                pixel_data.get_green(),
                pixel_data.get_blue(),
                pixel_data.get_alpha(),
            ]);
        });

    image.save(filepath)?;
    println!("INFO: Saved image to {:?}", filepath);

    Ok(())
}
