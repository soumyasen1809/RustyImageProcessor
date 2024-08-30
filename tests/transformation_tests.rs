use image_processor::core::{image::Images, pixel::Pixels};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

fn common_steup_simple() -> Images {
    let mut vec_pix: Vec<Pixels> = vec![Pixels::new(0, 0, 0, 0); 16];
    vec_pix.par_iter_mut().enumerate().for_each(|(idx, val)| {
        *val = Pixels::new(idx as u8 * 3, idx as u8 * 6, idx as u8 * 9, 255)
    });
    let img = Images::new(4, 4, 3, vec_pix);

    img
}

#[cfg(test)]
mod tests {
    use image_processor::{
        core::{image::Images, operations::Operation, pixel::Pixels},
        transformations::{
            crop::Crop,
            resize::{ResizeBilinearInterpolation, ResizeNearestNeighbour},
        },
    };

    use super::*;

    #[test]
    fn crop_test() {
        // Create a sample image
        let img = common_steup_simple();

        // Apply cropping operation
        let cropped_img = Crop::new((1, 1), 2, 2, &img).apply();

        // Assert the result
        let expected_img = Images::new(
            2,
            2,
            3,
            vec![
                Pixels::new(15, 30, 45, 255), // idx: 5
                Pixels::new(18, 36, 54, 255), // idx: 6
                Pixels::new(27, 54, 81, 255), // idx: 9
                Pixels::new(30, 60, 90, 255), // idx: 10
            ],
        );
        assert_eq!(
            cropped_img.get_image().len(),
            expected_img.get_image().len()
        );
        assert_eq!(cropped_img, expected_img);
    }

    #[test]
    fn resize_nearest_neighbour_test() {
        // Create a sample image
        let img = common_steup_simple();

        // Apply cropping operation
        let resized_img = ResizeNearestNeighbour::new(2, 2, &img).apply();

        // Assert the result
        let expected_img = Images::new(
            2,
            2,
            3,
            vec![
                Pixels::new(0, 0, 0, 255),
                Pixels::new(6, 12, 18, 255),
                Pixels::new(24, 48, 72, 255),
                Pixels::new(30, 60, 90, 255),
            ],
        );
        assert_eq!(resized_img, expected_img);
    }

    #[test]
    fn resize_bilinear_interpolation_test() {
        // Create a sample image
        let img = common_steup_simple();

        // Apply cropping operation
        let resized_img = ResizeBilinearInterpolation::new(2, 2, &img).apply();

        // Assert the result
        let expected_img = Images::new(
            2,
            2,
            3,
            vec![
                Pixels::new(0, 0, 0, 255),
                Pixels::new(6, 12, 18, 255),
                Pixels::new(24, 48, 72, 255),
                Pixels::new(30, 60, 90, 255),
            ],
        );
        assert_eq!(resized_img, expected_img);
    }
}
