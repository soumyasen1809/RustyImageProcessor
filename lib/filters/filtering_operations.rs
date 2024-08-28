use crate::core::{image::Images, operations::Operation};

use super::{
    blur::{Blur, SmoothingKernelChoices},
    edge_detection::{EdgeDetectingKernelChoices, EdgeDetection},
    gray_scale::{GrayScale, GrayScaleAlgorithms},
    sharpen::{Sharpen, SharpeningKernelChoices},
};

pub enum FilteringOperations {
    GrayScale(GrayScaleAlgorithms), // Takes GrayscaleAlgorithms as an input
    Smoothing(SmoothingKernelChoices),
    Sharpening(SharpeningKernelChoices),
    EdgeDetecting(EdgeDetectingKernelChoices),
}

impl FilteringOperations {
    pub fn chain_operations(image: &Images, operations: Vec<FilteringOperations>) -> Images {
        let mut new_image: Images = image.clone();

        for ops in operations.iter() {
            new_image = match ops {
                FilteringOperations::GrayScale(GrayScaleAlgorithms::Average) => {
                    GrayScale::new(&new_image, GrayScaleAlgorithms::Average).apply()
                }

                FilteringOperations::GrayScale(GrayScaleAlgorithms::Luminosity) => {
                    GrayScale::new(&new_image, GrayScaleAlgorithms::Luminosity).apply()
                }

                FilteringOperations::Smoothing(SmoothingKernelChoices::BoxBlur) => {
                    Blur::new(&new_image, SmoothingKernelChoices::BoxBlur).apply()
                }

                FilteringOperations::Smoothing(SmoothingKernelChoices::Gaussian) => {
                    Blur::new(&new_image, SmoothingKernelChoices::Gaussian).apply()
                }

                FilteringOperations::Sharpening(SharpeningKernelChoices::Basic) => {
                    Sharpen::new(&new_image, SharpeningKernelChoices::Basic).apply()
                }

                FilteringOperations::Sharpening(SharpeningKernelChoices::HighPass) => {
                    Sharpen::new(&new_image, SharpeningKernelChoices::HighPass).apply()
                }

                FilteringOperations::Sharpening(SharpeningKernelChoices::EdgeEnhancement) => {
                    Sharpen::new(&new_image, SharpeningKernelChoices::EdgeEnhancement).apply()
                }

                FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::Outline) => {
                    EdgeDetection::new(&new_image, EdgeDetectingKernelChoices::Outline).apply()
                }

                FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::SobelX) => {
                    EdgeDetection::new(&new_image, EdgeDetectingKernelChoices::SobelX).apply()
                }

                FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::SobelY) => {
                    EdgeDetection::new(&new_image, EdgeDetectingKernelChoices::SobelY).apply()
                }

                FilteringOperations::EdgeDetecting(EdgeDetectingKernelChoices::Emboss) => {
                    EdgeDetection::new(&new_image, EdgeDetectingKernelChoices::Emboss).apply()
                }
            };
        }

        new_image
    }
}
