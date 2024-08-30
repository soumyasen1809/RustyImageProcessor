use crate::core::image::Images;

pub fn rgba_to_hsv(image: Images) -> Vec<(f64, f64, f64)> {
    let mut hsv_image: Vec<(f64, f64, f64)> = Vec::new();
    for pix in image.get_image() {
        let r = pix.get_red() as f64;
        let g = pix.get_green() as f64;
        let b = pix.get_blue() as f64;

        let min_val = (r.min(g)).min(b);
        let max_val = (r.max(g)).max(b);
        let delta = max_val - min_val;

        let mut hsv: (f64, f64, f64) = (0.0, 0.0, 0.0);
        if delta == 0.0 {
            hsv.0 = 0.0;
            hsv.1 = 0.0;
            hsv.2 = max_val as f64;
        } else {
            if max_val == hsv.0 {
                hsv.0 = 60.0 * (g - b) / delta;
            } else if max_val == hsv.1 {
                hsv.0 = 60.0 * (b - r) / delta + 120.0;
            } else {
                hsv.0 = 60.0 * (r - g) / delta + 240.0;
            }

            if hsv.0 < 0.0 {
                hsv.0 += 360.0
            }

            hsv.1 = delta / max_val;
            hsv.2 = max_val;
        }

        hsv_image.push((hsv.0, hsv.1 * 255.0, hsv.2 * 255.0));
    }

    hsv_image
}
