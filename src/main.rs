use image::{GenericImageView, ImageReader, Pixel};
use image_processing_lib::{
    core::operations::Operation,
    filters::{
        blur::SmoothingKernelChoices, filtering_operations::FilteringOperations,
        gray_scale::GrayScaleAlgorithms, sharpen::Sharpen,
    },
    transformations::{
        crop::CroppingOperations, resize::ResizingOperations, rotate::RotatingOperations,
        transformation_operations::TransformationOperations,
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
        TransformationOperations::Rotate(RotatingOperations::RotateVertical),
        // TransformationOperations::Rotate(RotatingOperations::RotateHorizontal),
        TransformationOperations::Rotate(RotatingOperations::Rotate90Left),
        // TransformationOperations::Rotate(RotatingOperations::Rotate90Right),
    ];
    let flipped_image =
        TransformationOperations::chain_operations(&image_read.unwrap(), transform_operations);

    let resize_operations = vec![
        // TransformationOperations::Resize(ResizingOperations::NearestNeighbours(256, 256)),
        TransformationOperations::Resize(ResizingOperations::BilinearInterpolation(256, 256)), // Preferred
    ];
    let resized_image =
        TransformationOperations::chain_operations(&flipped_image, resize_operations);

    let grayscale_operations = vec![
        // FilteringOperations::GrayScale(GrayScaleAlgorithms::AVERAGE),
        FilteringOperations::GrayScale(GrayScaleAlgorithms::LUMINOSITY), // Preferred
    ];
    let grayscale_image =
        FilteringOperations::chain_operations(&resized_image, grayscale_operations);

    let blur_operation = vec![
        // FilteringOperations::Smoothing(SmoothingKernelChoices::BOXBLUR),
        FilteringOperations::Smoothing(SmoothingKernelChoices::GAUSSIAN), // Preferred
    ];
    let blur_image = FilteringOperations::chain_operations(&grayscale_image, blur_operation);

    let crop_operation = vec![TransformationOperations::Crop(
        CroppingOperations::SimpleCrop((50, 50), 128, 128),
    )];
    let cropped_image = TransformationOperations::chain_operations(&blur_image, crop_operation);

    let sharpen_operation = Sharpen::new(&cropped_image);
    let sharpened_image = Sharpen::apply(&sharpen_operation);

    let out_path = "assets/out_cropped.png";
    let image_write = image_writer(&out_path, &sharpened_image);

    image_write
}
