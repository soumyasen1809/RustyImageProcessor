use image::{GenericImageView, ImageReader, Pixel};
use image_processing_lib::{
    filters::{
        blur::BoxBlur,
        gray_scale::{FilteringOperations, GrayScaleAlgorithms},
    },
    transformations::{
        resize::ResizingOperations,
        rotate::{Transformation, TransformationOperations},
    },
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
        // TransformationOperations::FLIPVERTICAL,
        // TransformationOperations::FLIPHORIZONTAL,
        // TransformationOperations::FLIP90LEFT,
        // TransformationOperations::FLIP90RIGHT,
    ];
    let flipped_image =
        TransformationOperations::chain_operations(&image_read.unwrap(), transform_operations);

    let resize_operations = vec![
        // ResizingOperations::RESIZENEARESTNEIGHBOUR,
        ResizingOperations::RESIZEBILINEAR, // Preferred
    ];
    let resized_image =
        ResizingOperations::chain_operations(&flipped_image, resize_operations, 256, 256);

    let grayscale_operations = vec![
        // FilteringOperations::GrayScaleAlgorithm(GrayScaleAlgorithms::AVERAGE),
        FilteringOperations::GrayScaleAlgorithm(GrayScaleAlgorithms::LUMINOSITY), // Preferred
    ];
    let grayscale_image =
        FilteringOperations::chain_operations(&resized_image, grayscale_operations);

    let blur_operation = BoxBlur::new(&grayscale_image);
    let box_blur_image = blur_operation.apply();

    let out_path = "assets/out_cropped.png";
    let image_write = image_writer(&out_path, &box_blur_image);

    image_write
}
