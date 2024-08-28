use crate::core::{image::Images, operations::Operation};

use super::{
    blur::{Blur, SmoothingKernelChoices},
    gray_scale::{GrayScale, GrayScaleAlgorithms},
};

pub enum FilteringOperations {
    GrayScale(GrayScaleAlgorithms), // Takes GrayscaleAlgorithms as an input
    Smoothing(SmoothingKernelChoices),
}

impl FilteringOperations {
    pub fn chain_operations(image: &Images, operations: Vec<FilteringOperations>) -> Images {
        let mut new_image: Images = image.clone();

        for ops in operations.iter() {
            new_image = match ops {
                FilteringOperations::GrayScale(GrayScaleAlgorithms::AVERAGE) => {
                    GrayScale::new(&new_image, GrayScaleAlgorithms::AVERAGE).apply()
                }

                FilteringOperations::GrayScale(GrayScaleAlgorithms::LUMINOSITY) => {
                    GrayScale::new(&new_image, GrayScaleAlgorithms::LUMINOSITY).apply()
                }

                FilteringOperations::Smoothing(SmoothingKernelChoices::BOXBLUR) => {
                    Blur::new(&new_image, SmoothingKernelChoices::BOXBLUR).apply()
                }

                FilteringOperations::Smoothing(SmoothingKernelChoices::GAUSSIAN) => {
                    Blur::new(&new_image, SmoothingKernelChoices::GAUSSIAN).apply()
                }
            };
        }

        new_image
    }
}
