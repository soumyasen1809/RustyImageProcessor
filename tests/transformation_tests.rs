use image_processor::core::{image::Images, pixel::Pixels};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

fn common_setup_complex<T>() -> Images<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    let mut vec_pix: Vec<Pixels<T>> = vec![Pixels::new(0.into(), 0.into(), 0.into(), 0.into()); 16];
    vec_pix.par_iter_mut().enumerate().for_each(|(idx, val)| {
        *val = Pixels::new(
            (idx as u8 * 3).into(),
            (idx as u8 * 6).into(),
            (idx as u8 * 9).into(),
            (255 as u8).into(),
        )
    });

    Images::new(4, 4, 3, vec_pix)
}

fn common_setup_simple<T>() -> Images<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    let mut vec_pix: Vec<Pixels<T>> = vec![Pixels::new(0.into(), 0.into(), 0.into(), 0.into()); 4];
    vec_pix.par_iter_mut().enumerate().for_each(|(idx, val)| {
        *val = Pixels::new(
            (idx as u8 * 3).into(),
            (idx as u8 * 6).into(),
            (idx as u8 * 9).into(),
            (255 as u8).into(),
        )
    });

    Images::new(2, 2, 3, vec_pix)
}

#[cfg(test)]
mod tests {
    use image_processor::{
        core::{image::Images, operations::Operation, pixel::Pixels},
        transformations::{
            crop::Crop,
            resize::{ResizeBilinearInterpolation, ResizeNearestNeighbour},
            rotate::{Flip90Left, Flip90Right, FlipHorizontal, FlipVertical},
        },
    };

    use super::*;

    #[test]
    fn crop_test() {
        // Create a sample image
        let img = common_setup_complex();

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
        let img = common_setup_complex();

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
        let img = common_setup_complex();

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

    #[test]
    fn rotate_vertical_test() {
        // Create a sample image
        let img = common_setup_simple();

        // Apply cropping operation
        let resized_img = FlipVertical::new(&img).apply();

        // Assert the result
        let expected_img = Images::new(
            2,
            2,
            3,
            vec![
                Pixels::new(9, 18, 27, 255),
                Pixels::new(6, 12, 18, 255),
                Pixels::new(3, 6, 9, 255),
                Pixels::new(0, 0, 0, 255),
            ],
        );
        assert_eq!(resized_img, expected_img);
    }

    #[test]
    fn rotate_horizontal_test() {
        // Create a sample image
        let img = common_setup_simple();

        // Apply cropping operation
        let resized_img = FlipHorizontal::new(&img).apply();

        // Assert the result
        let expected_img = Images::new(
            2,
            2,
            3,
            vec![
                Pixels::new(3, 6, 9, 255),
                Pixels::new(0, 0, 0, 255),
                Pixels::new(9, 18, 27, 255),
                Pixels::new(6, 12, 18, 255),
            ],
        );
        assert_eq!(resized_img, expected_img);
    }

    #[test]
    fn rotate_left_test() {
        // Create a sample image
        let img = common_setup_simple();

        // Apply cropping operation
        let resized_img = Flip90Left::new(&img).apply();

        // Assert the result
        let expected_img = Images::new(
            2,
            2,
            3,
            vec![
                Pixels::new(6, 12, 18, 255),
                Pixels::new(0, 0, 0, 255),
                Pixels::new(9, 18, 27, 255),
                Pixels::new(3, 6, 9, 255),
            ],
        );
        assert_eq!(resized_img, expected_img);
    }

    #[test]
    fn rotate_right_test() {
        // Create a sample image
        let img = common_setup_simple();

        // Apply cropping operation
        let resized_img = Flip90Right::new(&img).apply();

        // Assert the result
        let expected_img = Images::new(
            2,
            2,
            3,
            vec![
                Pixels::new(0, 0, 0, 255),
                Pixels::new(6, 12, 18, 255),
                Pixels::new(3, 6, 9, 255),
                Pixels::new(9, 18, 27, 255),
            ],
        );
        assert_eq!(resized_img, expected_img);
    }
}
