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

pub fn chain_operations<T>(
    image: &Images<T>,
    operations: Vec<TransformationOperations>,
) -> Images<T>
where
    T: Copy + Clone + From<u8> + Into<f64> + std::cmp::PartialEq + Send + Sync,
{
    let mut new_image: Images<T> = image.clone();

    for ops in operations.iter() {
        new_image = match ops {
            TransformationOperations::Rotate(RotatingOperations::RotateVertical) => {
                println!("INFO: Rotating image Vertically");
                FlipVertical::new(&new_image).apply()
            }

            TransformationOperations::Rotate(RotatingOperations::RotateHorizontal) => {
                println!("INFO: Rotating image Horizontally");
                FlipHorizontal::new(&new_image).apply()
            }

            TransformationOperations::Rotate(RotatingOperations::Rotate90Left) => {
                println!("INFO: Rotating image 90 degrees Left");
                Flip90Left::new(&new_image).apply()
            }

            TransformationOperations::Rotate(RotatingOperations::Rotate90Right) => {
                println!("INFO: Rotating image 90 degree Right");
                Flip90Right::new(&new_image).apply()
            }

            TransformationOperations::Resize(ResizingOperations::BilinearInterpolation(
                new_width,
                new_height,
            )) => {
                println!("INFO: Resizing image using Bilinear Interpolation");
                ResizeBilinearInterpolation::new(*new_width, *new_height, &new_image).apply()
            }

            TransformationOperations::Resize(ResizingOperations::NearestNeighbours(
                new_width,
                new_height,
            )) => {
                println!("INFO: Resizing image with Nearest Neighbour");
                ResizeNearestNeighbour::new(*new_width, *new_height, &new_image).apply()
            }

            TransformationOperations::Crop(CroppingOperations::SimpleCrop(
                top_left_point,
                new_width,
                new_height,
            )) => {
                println!(
                    "INFO: Cropping image to size: {:?} X {:?}",
                    new_width, new_height
                );
                Crop::new(*top_left_point, *new_width, *new_height, image).apply()
            }
        };
    }

    new_image
}
