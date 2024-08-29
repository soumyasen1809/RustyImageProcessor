use image::{GenericImageView, ImageReader, Pixel};
use image_processing_lib::{
    filters::{
        blur::SmoothingKernelChoices, edge_detection::EdgeDetectingKernelChoices,
        filtering_operations::FilteringOperations, gray_scale::GrayScaleAlgorithms,
        sharpen::SharpeningKernelChoices,
    },
    transformations::{
        crop::CroppingOperations, resize::ResizingOperations, rotate::RotatingOperations,
        transformation_operations::TransformationOperations,
    },
    utils::{
        image_io::{image_reader, image_writer},
        statistics::{compute_histogram, compute_variance, print_histogram},
    },
};

const PATH: &str = "assets/lenna.png";
const OUT_PATH: &str = "assets/out_cropped.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image = ImageReader::open(PATH)?.decode()?;

    println!(
        "Last pixel in image is: {:?}",
        image
            .get_pixel(image.width() - 1, image.height() - 1)
            .channels()
            .get(2)
    );

    let image_read = image_reader(PATH);
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
        // FilteringOperations::GrayScale(GrayScaleAlgorithms::Average),
        FilteringOperations::GrayScale(GrayScaleAlgorithms::Luminosity), // Preferred
    ];
    let grayscale_image =
        FilteringOperations::chain_operations(&resized_image, grayscale_operations);

    let blur_operation = vec![
        // FilteringOperations::Smoothing(SmoothingKernelChoices::BoxBlur),
        FilteringOperations::Smoothing(SmoothingKernelChoices::Gaussian), // Preferred
    ];
    let blur_image = FilteringOperations::chain_operations(&grayscale_image, blur_operation);

    let crop_operation = vec![TransformationOperations::Crop(
        CroppingOperations::SimpleCrop((50, 50), 128, 128),
    )];
    let cropped_image = TransformationOperations::chain_operations(&blur_image, crop_operation);

    let edge_detection_operation = vec![
        // Ideally, use either sharpening or edge detection
        FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::Outline), // Preferred for EdgeDetecting
        // FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::SobelX),
        // FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::SobelY),
        // FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::Emboss),
        // FilteringOperations::Sharpening(SharpeningKernelChoices::Basic),
        // FilteringOperations::Sharpening(SharpeningKernelChoices::HighPass),
        FilteringOperations::Sharpening(SharpeningKernelChoices::EdgeEnhancement), // Preferred for Sharpening
    ];
    let edge_detected_image =
        FilteringOperations::chain_operations(&cropped_image, edge_detection_operation);

    let histogram_stats = compute_histogram(&resized_image);
    print_histogram(histogram_stats);

    let variance = compute_variance(&resized_image);
    println!("Variance for the resized image: {:?}", variance);

    let image_write = image_writer(&OUT_PATH, &edge_detected_image);

    image_write
}
