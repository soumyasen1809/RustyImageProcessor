use crate::core::{image::Images, operations::Operation};

use super::{
    crop::{Crop, CroppingOperations},
    resize::{ResizeBilinearInterpolation, ResizeNearestNeighbour, ResizingOperations},
    rotate::{Flip90Left, Flip90Right, FlipHorizontal, FlipVertical, RotatingOperations},
};

pub enum TransformationOperations {
    Rotate(RotatingOperations),
    Resize(ResizingOperations),
    Crop(CroppingOperations),
}

impl TransformationOperations {
    pub fn chain_operations(image: &Images, operations: Vec<TransformationOperations>) -> Images {
        let mut new_image: Images = image.clone();

        for ops in operations.iter() {
            new_image = match ops {
                TransformationOperations::Rotate(RotatingOperations::RotateVertical) => {
                    FlipVertical::new(&new_image).apply()
                }

                TransformationOperations::Rotate(RotatingOperations::RotateHorizontal) => {
                    FlipHorizontal::new(&new_image).apply()
                }

                TransformationOperations::Rotate(RotatingOperations::Rotate90Left) => {
                    Flip90Left::new(&new_image).apply()
                }

                TransformationOperations::Rotate(RotatingOperations::Rotate90Right) => {
                    Flip90Right::new(&new_image).apply()
                }

                TransformationOperations::Resize(ResizingOperations::BilinearInterpolation(
                    new_width,
                    new_height,
                )) => ResizeBilinearInterpolation::new(*new_width, *new_height, &new_image).apply(),

                TransformationOperations::Resize(ResizingOperations::NearestNeighbours(
                    new_width,
                    new_height,
                )) => ResizeNearestNeighbour::new(*new_width, *new_height, &new_image).apply(),

                TransformationOperations::Crop(CroppingOperations::SimpleCrop(
                    top_left_point,
                    new_width,
                    new_height,
                )) => Crop::new(*top_left_point, *new_width, *new_height, &image).apply(),
            };
        }

        new_image
    }
}
