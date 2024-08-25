use image::{GenericImageView, ImageReader, Pixel};
use image_processing_lib::{
    transformations::{resize::ResizingOperations, rotate::TransformationOperations},
    utils::image_io::{image_reader, image_writer},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "assets/lenna.png";
    let image = ImageReader::open(path)?.decode()?;

    println!(
        "image is: {:?}",
        image
            .get_pixel(image.width() - 1, image.height() - 1)
            .channels()
            .get(2)
    );

    let image_read = image_reader(path);
    let transform_operations = vec![
        TransformationOperations::FLIPVERTICAL,
        // TransformationOperations::FLIPHORIZONTAL,
        // TransformationOperations::FLIP90LEFT,
        TransformationOperations::FLIP90RIGHT,
    ];
    let flipped_image =
        TransformationOperations::chain_operations(&image_read.unwrap(), transform_operations);

    let resize_operations = vec![ResizingOperations::RESIZENEARESTNEIGHBOUR];
    let resized_image =
        ResizingOperations::chain_operations(&flipped_image, resize_operations, 128, 128);
    let out_path = "assets/out_cropped.png";
    let image_write = image_writer(&out_path, &resized_image);

    image_write
}
