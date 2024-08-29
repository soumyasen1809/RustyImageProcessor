#[cfg(test)]
mod tests {
    use image_processor::{
        core::{image::Images, operations::Operation, pixel::Pixels},
        filters::{
            blur::{Blur, SmoothingKernelChoices},
            edge_detection::{EdgeDetectingKernelChoices, EdgeDetection},
            gray_scale::{GrayScale, GrayScaleAlgorithms},
            sharpen::{Sharpen, SharpeningKernelChoices},
        },
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

    #[test]
    fn edge_detection_filter_outline_test() {
        let img = Images::new(
            3,
            3,
            3,
            vec![
                Pixels::new(155, 0, 0, 155),
                Pixels::new(0, 155, 155, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 155, 0, 155),
                Pixels::new(0, 155, 0, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 0, 155, 155),
            ],
        );

        let edge_detection_image =
            EdgeDetection::new(&img, EdgeDetectingKernelChoices::Outline).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(255, 255, 0, 255)]);
        assert_eq!(edge_detection_image, new_image);
    }

    #[test]
    fn edge_detection_filter_sobelx_test() {
        let img = Images::new(
            3,
            3,
            3,
            vec![
                Pixels::new(155, 0, 0, 155),
                Pixels::new(0, 155, 155, 155),
                Pixels::new(100, 120, 130, 255),
                Pixels::new(110, 125, 135, 255),
                Pixels::new(120, 130, 140, 255),
                Pixels::new(255, 0, 0, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 0, 255, 155),
                Pixels::new(155, 0, 155, 155),
            ],
        );

        let edge_detection_image =
            EdgeDetection::new(&img, EdgeDetectingKernelChoices::SobelX).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(255, 0, 0, 255)]);
        assert_eq!(edge_detection_image, new_image);
    }

    #[test]
    fn edge_detection_filter_sobely_test() {
        let img = Images::new(
            3,
            3,
            3,
            vec![
                Pixels::new(155, 0, 0, 155),
                Pixels::new(0, 155, 155, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 155, 0, 155),
                Pixels::new(0, 155, 0, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 0, 155, 155),
            ],
        );

        let edge_detection_image =
            EdgeDetection::new(&img, EdgeDetectingKernelChoices::SobelY).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(255, 0, 155, 255)]);
        assert_eq!(edge_detection_image, new_image);
    }

    #[test]
    fn edge_detection_filter_emboss_test() {
        let img = Images::new(
            3,
            3,
            3,
            vec![
                Pixels::new(155, 0, 0, 155),
                Pixels::new(0, 155, 155, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 155, 0, 155),
                Pixels::new(0, 155, 0, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 0, 155, 155),
            ],
        );

        let edge_detection_image =
            EdgeDetection::new(&img, EdgeDetectingKernelChoices::Emboss).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(255, 255, 255, 255)]);
        assert_eq!(edge_detection_image, new_image);
    }

    #[test]
    fn sharpen_filter_basic_test() {
        let img = Images::new(
            3,
            3,
            3,
            vec![
                Pixels::new(155, 0, 0, 155),
                Pixels::new(0, 155, 155, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 155, 0, 155),
                Pixels::new(0, 155, 0, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 0, 155, 155),
            ],
        );

        let sharpen_image = Sharpen::new(&img, SharpeningKernelChoices::Basic).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(255, 155, 0, 255)]);
        assert_eq!(sharpen_image, new_image);
    }

    #[test]
    fn sharpen_filter_highpass_test() {
        let img = Images::new(
            3,
            3,
            3,
            vec![
                Pixels::new(155, 0, 0, 155),
                Pixels::new(0, 155, 155, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 155, 0, 155),
                Pixels::new(0, 155, 0, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 0, 155, 155),
            ],
        );

        let sharpen_image = Sharpen::new(&img, SharpeningKernelChoices::HighPass).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(255, 255, 0, 255)]);
        assert_eq!(sharpen_image, new_image);
    }

    #[test]
    fn sharpen_filter_edge_enhancement_test() {
        let img = Images::new(
            3,
            3,
            3,
            vec![
                Pixels::new(155, 0, 0, 155),
                Pixels::new(0, 155, 155, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 155, 0, 155),
                Pixels::new(0, 155, 0, 155),
                Pixels::new(0, 0, 155, 155),
                Pixels::new(155, 155, 155, 155),
                Pixels::new(155, 0, 155, 155),
            ],
        );

        let sharpen_image = Sharpen::new(&img, SharpeningKernelChoices::EdgeEnhancement).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(255, 255, 0, 255)]);
        assert_eq!(sharpen_image, new_image);
    }

    #[test]
    fn grayscale_filter_average_test() {
        let img = Images::new(1, 1, 3, vec![Pixels::new(255, 200, 100, 255)]);

        let gray_image = GrayScale::new(&img, GrayScaleAlgorithms::Average).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(14, 14, 14, 255)]);
        assert_eq!(gray_image, new_image);
    }

    #[test]
    fn grayscale_filter_luminosity_test() {
        let img = Images::new(1, 1, 3, vec![Pixels::new(255, 200, 100, 255)]);

        let gray_image = GrayScale::new(&img, GrayScaleAlgorithms::Luminosity).apply();
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(68, 68, 68, 255)]);
        assert_eq!(gray_image, new_image);
    }
}
