use crate::core::{image::Images, operations::Operation};

use super::rotate::{Flip90Left, Flip90Right, FlipHorizontal, FlipVertical};

pub enum TransformationOperations {
    FLIPVERTICAL,
    FLIPHORIZONTAL,
    FLIP90LEFT,
    FLIP90RIGHT,
}

impl TransformationOperations {
    pub fn chain_operations(image: &Images, operations: Vec<TransformationOperations>) -> Images {
        let mut new_image: Images = image.clone();

        for ops in operations.iter() {
            new_image = match ops {
                TransformationOperations::FLIPVERTICAL => FlipVertical::new(&new_image).apply(),
                TransformationOperations::FLIPHORIZONTAL => FlipHorizontal::new(&new_image).apply(),
                TransformationOperations::FLIP90LEFT => Flip90Left::new(&new_image).apply(),
                TransformationOperations::FLIP90RIGHT => Flip90Right::new(&new_image).apply(),
            };
        }

        new_image
    }
}
