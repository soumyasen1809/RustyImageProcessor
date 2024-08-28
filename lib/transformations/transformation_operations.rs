use crate::core::{image::Images, operations::Operation};

use super::{
    resize::{ResizeBilinearInterpolation, ResizeNearestNeighbour, ResizingOperations},
    rotate::{Flip90Left, Flip90Right, FlipHorizontal, FlipVertical},
};

pub enum TransformationOperations {
    RotateVertical,
    RotateHorizontal,
    Rotate90Left,
    Rotate90Right,
    Resize(ResizingOperations),
}

impl TransformationOperations {
    pub fn chain_operations(image: &Images, operations: Vec<TransformationOperations>) -> Images {
        let mut new_image: Images = image.clone();

        for ops in operations.iter() {
            new_image = match ops {
                TransformationOperations::RotateVertical => FlipVertical::new(&new_image).apply(),

                TransformationOperations::RotateHorizontal => {
                    FlipHorizontal::new(&new_image).apply()
                }

                TransformationOperations::Rotate90Left => Flip90Left::new(&new_image).apply(),

                TransformationOperations::Rotate90Right => Flip90Right::new(&new_image).apply(),

                TransformationOperations::Resize(ResizingOperations::BilinearInterpolation(
                    new_width,
                    new_height,
                )) => ResizeBilinearInterpolation::new(*new_width, *new_height, &new_image).apply(),

                TransformationOperations::Resize(ResizingOperations::NearestNeighbours(
                    new_width,
                    new_height,
                )) => ResizeNearestNeighbour::new(*new_width, *new_height, &new_image).apply(),
            };
        }

        new_image
    }
}
