use image::{GenericImageView, ImageReader, Pixel};
use image_processing_lib::{
    transformations::rotate::{FlipHorizontal, Transformation},
    utils::image_io::{image_reader, image_writer},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "assets/cropped.jpg";
    let image = ImageReader::open(path)?.decode()?;

    println!(
        "image is: {:?}",
        image
            .get_pixel(image.width() - 1, image.height() - 1)
            .channels()
            .get(2)
    );

    let image_read = image_reader(path);
    let flipped_image = FlipHorizontal::new(&image_read.unwrap()).apply();
    let out_path = "assets/out_cropped.png";
    let _image_write = image_writer(&out_path, &flipped_image);

    Ok(())
}
