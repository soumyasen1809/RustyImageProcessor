#[cfg(test)]
mod tests {
    use image_processor::{
        core::{image::Images, operations::Operation, pixel::Pixels},
        filters::blur::{Blur, SmoothingKernelChoices},
    };

    #[test]
    fn blur_filter_box_blur_test() {
        let pix = Pixels::new(100, 100, 100, 255);
        let img = Images::new(3, 3, 3, vec![pix; 9]);
        let blurred_img = Blur::new(&img, SmoothingKernelChoices::BoxBlur).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(44, 44, 44, 255)]);
        assert_eq!(blurred_img, new_image);
    }

    #[test]
    fn blur_filter_box_blur_bigger_test() {
        // Create a sample image
        let img = Images::new(
            3,
            3,
            3,
            vec![
                Pixels::new(255, 0, 0, 255),
                Pixels::new(0, 255, 0, 255),
                Pixels::new(0, 0, 255, 255),
                Pixels::new(255, 255, 255, 255),
                Pixels::new(255, 0, 0, 255),
                Pixels::new(0, 255, 0, 255),
                Pixels::new(0, 0, 255, 255),
                Pixels::new(255, 255, 255, 255),
                Pixels::new(255, 0, 255, 255),
            ],
        );

        // Apply the blur filter
        let blurred_img = Blur::new(&img, SmoothingKernelChoices::BoxBlur).apply();

        // Assert the result
        let expected_img = Images::new(1, 1, 3, vec![Pixels::new(85, 56, 28, 255)]);
        assert_eq!(blurred_img, expected_img);
    }

    #[test]
    fn blur_filter_gaussian_kernel_test() {
        let pix = Pixels::new(100, 100, 100, 255);
        let img = Images::new(3, 3, 3, vec![pix; 9]);

        let blurred_img = Blur::new(&img, SmoothingKernelChoices::Gaussian).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(56, 56, 56, 255)]);
        assert_eq!(blurred_img, new_image);
    }

    #[test]
    fn blur_filter_gaussian_kernel_bigger_test() {
        // Create a sample image
        let img = Images::new(
            3,
            3,
            3,
            vec![
                Pixels::new(255, 0, 0, 255),
                Pixels::new(0, 255, 0, 255),
                Pixels::new(0, 0, 255, 255),
                Pixels::new(255, 255, 255, 255),
                Pixels::new(255, 0, 0, 255),
                Pixels::new(0, 255, 0, 255),
                Pixels::new(0, 0, 255, 255),
                Pixels::new(255, 255, 255, 255),
                Pixels::new(255, 0, 255, 255),
            ],
        );

        // Apply the blur filter
        let blurred_img = Blur::new(&img, SmoothingKernelChoices::Gaussian).apply();

        // Assert the result
        let expected_img = Images::new(1, 1, 3, vec![Pixels::new(111, 63, 31, 255)]);
        assert_eq!(blurred_img, expected_img);
    }
}
