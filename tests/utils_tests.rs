const TOL: f64 = 0.001;

#[cfg(test)]
mod tests {
    use image_processor::{
        core::{image::Images, pixel::Pixels},
        utils::color_space_converter::rgba_to_hsv,
    };

    use super::*;

    #[test]
    fn rgba_to_hsv_test() {
        let rgba_image = Images::new(1, 1, 3, vec![Pixels::new(10, 20, 30, 255)]);

        let computed_hsv_image = rgba_to_hsv(rgba_image);
        let expected_hsv_image = [(210.0, 0.67, 0.12)];
        assert!(computed_hsv_image[0].0 - expected_hsv_image[0].0 < TOL);
        assert!(computed_hsv_image[0].1 - expected_hsv_image[0].1 < TOL);
        assert!(computed_hsv_image[0].2 - expected_hsv_image[0].2 < TOL);
    }
}
