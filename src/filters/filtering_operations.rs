use crate::{
    core::{image::Images, operations::Operation},
    filters::morphological::{Dilation, Erosion},
};

use super::{
    blur::{Blur, SmoothingKernelChoices},
    edge_detection::{EdgeDetectingKernelChoices, EdgeDetection},
    gray_scale::{GrayScale, GrayScaleAlgorithms},
    morphological::MorphologicalOperations,
    sharpen::{Sharpen, SharpeningKernelChoices},
};

pub enum FilteringOperations {
    GrayScale(GrayScaleAlgorithms), // Takes GrayscaleAlgorithms as an input
    Smoothing(SmoothingKernelChoices),
    Sharpening(SharpeningKernelChoices),
    EdgeDetecting(EdgeDetectingKernelChoices),
    Morphological(MorphologicalOperations),
}

impl FilteringOperations {
    pub fn chain_operations(image: &Images, operations: Vec<FilteringOperations>) -> Images {
        let mut new_image: Images = image.clone();

        for ops in operations.iter() {
            new_image = match ops {
                FilteringOperations::GrayScale(GrayScaleAlgorithms::Average) => {
                    println!("INFO: Applying Filter Grayscale");
                    GrayScale::new(&new_image, GrayScaleAlgorithms::Average).apply()
                }

                FilteringOperations::GrayScale(GrayScaleAlgorithms::Luminosity) => {
                    println!("INFO: Applying Filter Grayscale");
                    GrayScale::new(&new_image, GrayScaleAlgorithms::Luminosity).apply()
                }

                FilteringOperations::Smoothing(SmoothingKernelChoices::BoxBlur) => {
                    println!("INFO: Smoothing image with Box Blur");
                    Blur::new(&new_image, SmoothingKernelChoices::BoxBlur).apply()
                }

                FilteringOperations::Smoothing(SmoothingKernelChoices::Gaussian) => {
                    println!("INFO: Smoothing image with Gaussian");
                    Blur::new(&new_image, SmoothingKernelChoices::Gaussian).apply()
                }

                FilteringOperations::Sharpening(SharpeningKernelChoices::Basic) => {
                    println!("INFO: Sharpening image");
                    Sharpen::new(&new_image, SharpeningKernelChoices::Basic).apply()
                }

                FilteringOperations::Sharpening(SharpeningKernelChoices::HighPass) => {
                    println!("INFO: Sharpening image with High Pass filter");
                    Sharpen::new(&new_image, SharpeningKernelChoices::HighPass).apply()
                }

                FilteringOperations::Sharpening(SharpeningKernelChoices::EdgeEnhancement) => {
                    println!("INFO: Sharpening image with edge enhancement");
                    Sharpen::new(&new_image, SharpeningKernelChoices::EdgeEnhancement).apply()
                }

                FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::Outline) => {
                    println!("INFO: Edge detecting in image with outline");
                    EdgeDetection::new(&new_image, EdgeDetectingKernelChoices::Outline).apply()
                }

                FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::SobelX) => {
                    println!("INFO: Edge detecting in image with Sobel X");
                    EdgeDetection::new(&new_image, EdgeDetectingKernelChoices::SobelX).apply()
                }

                FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::SobelY) => {
                    println!("INFO: Edge detecting in image with Sobel Y");
                    EdgeDetection::new(&new_image, EdgeDetectingKernelChoices::SobelY).apply()
                }

                FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::Emboss) => {
                    println!("INFO: Edge detecting in image with Emboss");
                    EdgeDetection::new(&new_image, EdgeDetectingKernelChoices::Emboss).apply()
                }

                FilteringOperations::Morphological(MorphologicalOperations::Erode(
                    morphological_kernel_choices,
                )) => {
                    println!("INFO: Eroding image");
                    Erosion::new(&new_image, *morphological_kernel_choices).apply()
                }

                FilteringOperations::Morphological(MorphologicalOperations::Dialate(
                    morphological_kernel_choices,
                )) => {
                    println!("INFO: Dialating image");
                    Dilation::new(&new_image, *morphological_kernel_choices).apply()
                }
            };
        }

        new_image
    }
}
