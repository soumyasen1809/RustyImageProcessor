use image_processor::core::{image::Images, pixel::Pixels};

fn common_steup_simple<T>() -> (Pixels<T>, Images<T>)
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    let pix: Pixels<T> = Pixels::new(100_u8.into(), 100_u8.into(), 100_u8.into(), 255_u8.into());
    let img: Images<T> = Images::new(3, 3, 3, vec![pix.clone(); 9]);

    (pix, img)
}

fn common_steup_complex<T>() -> (Pixels<T>, Images<T>)
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    let pix: Pixels<T> = Pixels::new(100_u8.into(), 100_u8.into(), 100_u8.into(), 255_u8.into());
    let img = Images::new(
        3,
        3,
        3,
        vec![
            Pixels::new(255_u8.into(), 0_u8.into(), 0_u8.into(), 255_u8.into()),
            Pixels::new(0_u8.into(), 255_u8.into(), 0_u8.into(), 255_u8.into()),
            Pixels::new(0_u8.into(), 0_u8.into(), 255_u8.into(), 255_u8.into()),
            Pixels::new(255_u8.into(), 255_u8.into(), 255_u8.into(), 255_u8.into()),
            Pixels::new(255_u8.into(), 0_u8.into(), 0_u8.into(), 255_u8.into()),
            Pixels::new(0_u8.into(), 255_u8.into(), 0_u8.into(), 255_u8.into()),
            Pixels::new(0_u8.into(), 0_u8.into(), 255_u8.into(), 255_u8.into()),
            Pixels::new(255_u8.into(), 255_u8.into(), 255_u8.into(), 255_u8.into()),
            Pixels::new(255_u8.into(), 0_u8.into(), 255_u8.into(), 255_u8.into()),
        ],
    );

    (pix, img)
}

fn common_steup_complex_varied<T>() -> (Pixels<T>, Images<T>)
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    let pix: Pixels<T> = Pixels::new(100_u8.into(), 100_u8.into(), 100_u8.into(), 255_u8.into());
    let img: Images<T> = Images::new(
        3,
        3,
        3,
        vec![
            Pixels::new(155_u8.into(), 0_u8.into(), 0_u8.into(), 155_u8.into()),
            Pixels::new(0_u8.into(), 155_u8.into(), 155_u8.into(), 155_u8.into()),
            Pixels::new(100_u8.into(), 120_u8.into(), 130_u8.into(), 255_u8.into()),
            Pixels::new(110_u8.into(), 125_u8.into(), 135_u8.into(), 255_u8.into()),
            Pixels::new(120_u8.into(), 130_u8.into(), 140_u8.into(), 255_u8.into()),
            Pixels::new(255_u8.into(), 0_u8.into(), 0_u8.into(), 155_u8.into()),
            Pixels::new(0_u8.into(), 0_u8.into(), 155_u8.into(), 155_u8.into()),
            Pixels::new(155_u8.into(), 0_u8.into(), 255_u8.into(), 155_u8.into()),
            Pixels::new(155_u8.into(), 0_u8.into(), 155_u8.into(), 155_u8.into()),
        ],
    );

    (pix, img)
}

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

    use super::*;

    #[test]
    fn blur_filter_box_blur_test() {
        let (_, img) = common_steup_simple();

        let blurred_img: Images<u8> = Blur::new(SmoothingKernelChoices::BoxBlur).apply(&img);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(44, 44, 44, 255)]);
        assert_eq!(blurred_img, new_image);
    }

    #[test]
    fn blur_filter_box_blur_bigger_test() {
        // Create a sample image
        let (_, img) = common_steup_complex();

        // Apply the blur filter
        let blurred_img: Images<u8> = Blur::new(SmoothingKernelChoices::BoxBlur).apply(&img);

        // Assert the result
        let expected_img = Images::new(1, 1, 3, vec![Pixels::new(85, 56, 28, 255)]);
        assert_eq!(blurred_img, expected_img);
    }

    #[test]
    fn blur_filter_gaussian_kernel_test() {
        let (_, img) = common_steup_simple();

        let blurred_img: Images<u8> = Blur::new(SmoothingKernelChoices::Gaussian).apply(&img);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(56, 56, 56, 255)]);
        assert_eq!(blurred_img, new_image);
    }

    #[test]
    fn blur_filter_gaussian_kernel_bigger_test() {
        // Create a sample image
        let (_, img) = common_steup_complex();

        // Apply the blur filter
        let blurred_img: Images<u8> = Blur::new(SmoothingKernelChoices::Gaussian).apply(&img);

        // Assert the result
        let expected_img = Images::new(1, 1, 3, vec![Pixels::new(111, 63, 31, 255)]);
        assert_eq!(blurred_img, expected_img);
    }

    #[test]
    fn edge_detection_filter_outline_test() {
        let (_, img) = common_steup_complex_varied();

        let edge_detection_image: Images<u8> =
            EdgeDetection::new(EdgeDetectingKernelChoices::Outline).apply(&img);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(30, 255, 135, 255)]);
        assert_eq!(edge_detection_image, new_image);
    }

    #[test]
    fn edge_detection_filter_sobelx_test() {
        let (_, img) = common_steup_complex_varied();

        let edge_detection_image: Images<u8> =
            EdgeDetection::new(EdgeDetectingKernelChoices::SobelX).apply(&img);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(255, 0, 0, 255)]);
        assert_eq!(edge_detection_image, new_image);
    }

    #[test]
    fn edge_detection_filter_sobely_test() {
        let (_, img) = common_steup_complex_varied();

        let edge_detection_image: Images<u8> =
            EdgeDetection::new(EdgeDetectingKernelChoices::SobelY).apply(&img);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(210, 0, 255, 255)]);
        assert_eq!(edge_detection_image, new_image);
    }

    #[test]
    fn edge_detection_filter_emboss_test() {
        let (_, img) = common_steup_complex_varied();

        let edge_detection_image: Images<u8> =
            EdgeDetection::new(EdgeDetectingKernelChoices::Emboss).apply(&img);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(255, 0, 255, 255)]);
        assert_eq!(edge_detection_image, new_image);
    }

    #[test]
    fn sharpen_filter_basic_test() {
        let (_, img) = common_steup_complex_varied();

        let sharpen_image: Images<u8> = Sharpen::new(SharpeningKernelChoices::Basic).apply(&img);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(80, 255, 155, 255)]);
        assert_eq!(sharpen_image, new_image);
    }

    #[test]
    fn sharpen_filter_highpass_test() {
        let (_, img) = common_steup_complex_varied();

        let sharpen_image: Images<u8> = Sharpen::new(SharpeningKernelChoices::HighPass).apply(&img);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(30, 255, 135, 255)]);
        assert_eq!(sharpen_image, new_image);
    }

    #[test]
    fn sharpen_filter_edge_enhancement_test() {
        let (_, img) = common_steup_complex_varied();

        let sharpen_image: Images<u8> =
            Sharpen::new(SharpeningKernelChoices::EdgeEnhancement).apply(&img);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(150, 255, 255, 255)]);
        assert_eq!(sharpen_image, new_image);
    }

    #[test]
    fn grayscale_filter_average_test() {
        let img = Images::new(1, 1, 3, vec![Pixels::new(255, 200, 90, 255)]);

        let gray_image: Images<u8> = GrayScale::new(GrayScaleAlgorithms::Average).apply(&img);
        println!("{:?}", gray_image);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(181, 181, 181, 255)]);
        assert_eq!(gray_image, new_image);
    }

    #[test]
    fn grayscale_filter_luminosity_test() {
        let img = Images::new(1, 1, 3, vec![Pixels::new(255, 200, 100, 255)]);

        let gray_image: Images<u8> = GrayScale::new(GrayScaleAlgorithms::Luminosity).apply(&img);
        let new_image = Images::new(1, 1, 3, vec![Pixels::new(68, 68, 68, 255)]);
        assert_eq!(gray_image, new_image);
    }
}
