#[cfg(test)]
mod tests {
    use image_processor::core::pixel::Pixels;

    use super::*;

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
}
