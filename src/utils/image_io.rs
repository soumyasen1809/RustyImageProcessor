use image::{load_from_memory, GenericImageView, Pixel, Rgba};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::{fs::File, io::AsyncReadExt};

use crate::core::{image::Images, pixel::Pixels};

pub async fn image_reader<T>(filepath: &str) -> Result<Images<T>, Box<dyn std::error::Error>>
where
    T: Copy + Clone + From<u8> + Into<u8> + std::cmp::PartialEq + Send + Sync,
{
    let mut file = File::open(filepath).await?;
    println!("INFO: Starting to read image from {:?} . . .", filepath);
    let mut tokio_image_bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut tokio_image_bytes).await?; // Reads all bytes until EOF, and places them into buf.
    let read_image = load_from_memory(&tokio_image_bytes)?;

    let mut image_bytes: Vec<Pixels<T>> = Vec::new();

    let read_pixels = (0..read_image.height())
        .into_par_iter()
        .flat_map(|h_index| {
            (0..read_image.width())
                .into_par_iter()
                .map(|w_index| {
                    let pixel = read_image.get_pixel(w_index, h_index).to_rgba();
                    Pixels::new(
                        (*pixel.channels().first().unwrap()).into(),
                        (*pixel.channels().get(1).unwrap()).into(),
                        (*pixel.channels().get(2).unwrap()).into(),
                        (*pixel.channels().get(3).unwrap()).into(),
                    )
                })
                .collect::<Vec<Pixels<T>>>()
        })
        .collect::<Vec<Pixels<T>>>();

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

pub fn image_writer<T>(
    filepath: &str,
    write_image: &Images<T>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Copy + Clone + From<u8> + Into<u8> + std::cmp::PartialEq + Send + Sync,
{
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
                pixel_data.get_red().into(),
                pixel_data.get_green().into(),
                pixel_data.get_blue().into(),
                pixel_data.get_alpha().into(),
            ]);
        });

    image.save(filepath)?;
    println!("INFO: Saved image to {:?}", filepath);

    Ok(())
}
