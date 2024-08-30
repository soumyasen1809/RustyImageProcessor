#[cfg(test)]
mod tests {
    use image_processor::core::{image::Images, pixel::Pixels};

    #[test]
    fn pixel_add_test() {
        let pix1 = Pixels::new(10, 20, 30, 255);
        let pix2 = Pixels::new(50, 60, 70, 255);

        let pix_computed = pix1 + pix2;
        let expected_pix = Pixels::new(60, 80, 100, 255);

        assert_eq!(pix_computed, expected_pix);
    }

    #[test]
    fn pixel_sub_test() {
        let pix1 = Pixels::new(10, 20, 30, 255);
        let pix2 = Pixels::new(50, 60, 70, 155);

        let pix_computed = pix2 - pix1;
        let expected_pix = Pixels::new(40, 40, 40, 0);

        assert_eq!(pix_computed, expected_pix);
    }

    #[test]
    fn pixel_mulf64_test() {
        let pix1 = Pixels::new(10, 20, 30, 255);
        let num = 10.0;

        let pix_computed = pix1 * num;
        let expected_pix = Pixels::new(100, 200, 255, 255);

        assert_eq!(pix_computed, expected_pix);
    }

    #[test]
    fn image_get_pixel_at_location_test() {
        let img = Images::new(
            2,
            2,
            3,
            vec![
                Pixels::new(10, 20, 30, 255),
                Pixels::new(40, 50, 60, 255),
                Pixels::new(70, 80, 90, 255),
                Pixels::new(100, 110, 120, 255),
            ],
        );

        let pix_computed = img.get_pixel_at(1, 1).unwrap_or_default();
        let expected_pix = Pixels::new(100, 110, 120, 255);

        assert_eq!(pix_computed, expected_pix);
    }

    #[test]
    fn image_get_pixel_at_location_out_of_bounds_test() {
        let img = Images::new(
            2,
            2,
            3,
            vec![
                Pixels::new(10, 20, 30, 255),
                Pixels::new(40, 50, 60, 255),
                Pixels::new(70, 80, 90, 255),
                Pixels::new(100, 110, 120, 255),
            ],
        );

        let pix_computed = img.get_pixel_at(4, 4).unwrap_or_default();
        let expected_pix = Pixels::new(0, 0, 0, 255);

        assert_eq!(pix_computed, expected_pix);
    }
}
