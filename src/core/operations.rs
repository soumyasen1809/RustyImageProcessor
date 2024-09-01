use std::{
    error::Error,
    path::{Path, PathBuf},
};

use image::{GenericImageView, ImageReader, Pixel};
use tokio::fs;

use crate::{
    filters::{
        blur::SmoothingKernelChoices,
        edge_detection::EdgeDetectingKernelChoices,
        filtering_operations::{self, FilteringOperations},
        gamma_correction::GammaCorrectionChoice,
        gray_scale::GrayScaleAlgorithms,
        morphological::{MorphologicalKernelChoices, MorphologicalOperations},
        sharpen::SharpeningKernelChoices,
    },
    transformations::{
        crop::CroppingOperations,
        resize::ResizingOperations,
        rotate::RotatingOperations,
        transformation_operations::{self, TransformationOperations},
    },
    utils::{
        image_io::{image_reader, image_writer},
        statistics::{compute_histogram, compute_mean, compute_variance, print_histogram},
    },
};

use super::image::Images;

pub trait Operation<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    fn apply(&self) -> Images<T>;
}

pub async fn process_images(
    is_dir: bool,
    dir: &str,
    dir_out: &str,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut dir_entries = fs::read_dir(dir).await?;
    match is_dir {
        true => {
            while let Some(entry) = dir_entries.next_entry().await? {
                let img_path = entry.path();
                let image = ImageReader::open(img_path.clone())?.decode()?;

                println!(
                    "Last pixel in image is: {:?}",
                    image
                        .get_pixel(image.width() - 1, image.height() - 1)
                        .channels()
                        .get(2)
                );

                let final_image = computation_image_processing(img_path.clone()).await?;

                let mut out_file_name = Path::new(img_path.to_str().unwrap())
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace("\"", "");

                out_file_name = String::from(dir_out) + &out_file_name + &String::from(".png");
                println!("Writing to: {:?}", out_file_name);
                image_writer((out_file_name).as_str(), &final_image)?;
            }
        }
        false => {
            let image = ImageReader::open(path)?.decode()?;

            println!(
                "Last pixel in image is: {:?}",
                image
                    .get_pixel(image.width() - 1, image.height() - 1)
                    .channels()
                    .get(2)
            );

            let final_image = computation_image_processing(PathBuf::from(path)).await?;

            let mut out_file_name = Path::new(path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .replace("\"", "");
            out_file_name = String::from(dir_out) + &out_file_name + &String::from(".png");
            println!("Writing to: {:?}", out_file_name);
            image_writer(&out_file_name, &final_image)?;
        }
    }

    Ok(())
}

pub async fn computation_image_processing(
    img_path: PathBuf,
) -> Result<Images<u8>, Box<dyn std::error::Error>> {
    let image_read: Result<Images<u8>, Box<dyn Error>> =
        image_reader(img_path.to_str().unwrap()).await;
    let transform_operations = vec![
        // TransformationOperations::Rotate(RotatingOperations::RotateVertical),
        TransformationOperations::Rotate(RotatingOperations::RotateHorizontal),
        TransformationOperations::Rotate(RotatingOperations::Rotate90Left),
        // TransformationOperations::Rotate(RotatingOperations::Rotate90Right),
    ];
    let flipped_image =
        transformation_operations::chain_operations(&image_read.unwrap(), transform_operations);

    let resize_operations = vec![
        // TransformationOperations::Resize(ResizingOperations::NearestNeighbours(256, 256)),
        TransformationOperations::Resize(ResizingOperations::BilinearInterpolation(256, 256)), // Preferred
    ];
    let resized_image =
        transformation_operations::chain_operations(&flipped_image, resize_operations);

    let grayscale_operations = vec![
        // FilteringOperations::GrayScale(GrayScaleAlgorithms::Average),
        FilteringOperations::GrayScale(GrayScaleAlgorithms::Luminosity), // Preferred
    ];
    let grayscale_image =
        filtering_operations::chain_operations(&resized_image, grayscale_operations);

    let blur_operation = vec![
        // FilteringOperations::Smoothing(SmoothingKernelChoices::BoxBlur),
        FilteringOperations::Smoothing(SmoothingKernelChoices::Gaussian), // Preferred
    ];
    let blur_image = filtering_operations::chain_operations(&grayscale_image, blur_operation);

    let crop_operation = vec![TransformationOperations::Crop(
        CroppingOperations::SimpleCrop((50, 50), 128, 128),
    )];
    let cropped_image = transformation_operations::chain_operations(&blur_image, crop_operation);

    let edge_detection_operation = vec![
        // Ideally, use either sharpening or edge detection
        // FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::Outline), // Preferred for EdgeDetecting
        // FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::SobelX),
        // FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::SobelY),
        FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::Emboss),
        // FilteringOperations::Sharpening(SharpeningKernelChoices::Basic),
        // FilteringOperations::Sharpening(SharpeningKernelChoices::HighPass),
        FilteringOperations::Sharpening(SharpeningKernelChoices::EdgeEnhancement), // Preferred for Sharpening
    ];
    let edge_detected_image =
        filtering_operations::chain_operations(&cropped_image, edge_detection_operation);

    let morphing_operations = vec![
        FilteringOperations::Morphological(MorphologicalOperations::Erode(
            MorphologicalKernelChoices::CrossKernel,
        )),
        // FilteringOperations::Morphological(MorphologicalOperations::Erode(
        //     MorphologicalKernelChoices::HorizontalKernel,
        // )),
        // FilteringOperations::Morphological(MorphologicalOperations::Erode(
        //     MorphologicalKernelChoices::DiagonalKernel,
        // )),
        // FilteringOperations::Morphological(MorphologicalOperations::Dialate(
        //     MorphologicalKernelChoices::VerticalKernel,
        // )),
        FilteringOperations::Morphological(MorphologicalOperations::Dialate(
            MorphologicalKernelChoices::DiamondKernel,
        )),
        // FilteringOperations::Morphological(MorphologicalOperations::Dialate(
        //     MorphologicalKernelChoices::DiagonalKernel2,
        // )),
    ];

    let morphed_image =
        filtering_operations::chain_operations(&edge_detected_image, morphing_operations);

    let gamma_operations = vec![FilteringOperations::GammaCorrecting(
        GammaCorrectionChoice::SimpleGamma(1.5),
    )];

    let gamma_image = filtering_operations::chain_operations(&morphed_image, gamma_operations);

    let histogram_stats = compute_histogram(&gamma_image);
    print_histogram(histogram_stats);

    let mean = compute_mean(&gamma_image);
    println!("Mean for the resized image: {:?}", mean);

    let variance = compute_variance(&gamma_image);
    println!("Variance for the resized image: {:?}", variance);

    Ok(gamma_image)
}
