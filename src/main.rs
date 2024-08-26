use image::{GenericImageView, ImageReader, Pixel};
use image_processing_lib::{
    filters::gray_scale::GrayScale,
    transformations::{
        resize::ResizingOperations,
        rotate::{Transformation, TransformationOperations},
    },
    utils::image_io::{image_reader, image_writer},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "assets/Cube.png";
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
        // TransformationOperations::FLIPVERTICAL,
        TransformationOperations::FLIPHORIZONTAL,
        // TransformationOperations::FLIP90LEFT,
        // TransformationOperations::FLIP90RIGHT,
    ];
    let flipped_image =
        TransformationOperations::chain_operations(&image_read.unwrap(), transform_operations);

    let resize_operations = vec![
        // ResizingOperations::RESIZENEARESTNEIGHBOUR,
        ResizingOperations::RESIZEBILINEAR,
    ];
    let resized_image =
        ResizingOperations::chain_operations(&flipped_image, resize_operations, 64, 64);

    let grayscale_operation = GrayScale::new(&resized_image);
    let grayscale_image = grayscale_operation.apply();

    let out_path = "assets/out_cropped.png";
    let image_write = image_writer(&out_path, &grayscale_image);

    image_write
}
