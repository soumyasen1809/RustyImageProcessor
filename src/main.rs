use image::{GenericImageView, ImageReader, Pixel};
use image_processing_lib::{
    transformations::rotate::{
        OperationsTypes,
        // Flip90Left, Flip90Right, FlipHorizontal, FlipVertical, Transformation,
    },
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
    // let flipped_image = FlipVertical::new(&image_read.unwrap()).apply();
    // let flipped_image = Flip90Right::new(&flipped_image).apply();
    // let flipped_image = FlipHorizontal::new(&flipped_image).apply();
    // let flipped_image = Flip90Left::new(&flipped_image).apply();
    let operations = vec![
        // OperationsTypes::FLIPVERTICAL,
        OperationsTypes::FLIPHORIZONTAL,
        // OperationsTypes::FLIP90LEFT,
        OperationsTypes::FLIP90RIGHT,
    ];
    let flipped_image = OperationsTypes::chain_operations(&image_read.unwrap(), operations);
    let out_path = "assets/out_cropped.png";
    let image_write = image_writer(&out_path, &flipped_image);

    image_write
}
