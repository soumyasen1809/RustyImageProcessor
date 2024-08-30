use crate::core::{image::Images, operations::Operation, pixel::Pixels};

pub struct GammaCorrection {
    image: Images,
    gamma: f64,
}

impl GammaCorrection {
    pub fn new(image: &Images, gamma: f64) -> Self {
        Self {
            image: image.clone(),
            gamma,
        }
    }
}

impl Operation for GammaCorrection {
    fn apply(&self) -> Images {
        let new_image: Vec<Pixels> = self
            .image
            .get_image()
            .iter()
            .map(|pix| {
                let r = (((pix.get_red() as f64) / 255.0).powf(self.gamma) * 255.0) as u8;
                let g = (((pix.get_green() as f64) / 255.0).powf(self.gamma) * 255.0) as u8;
                let b = (((pix.get_blue() as f64) / 255.0).powf(self.gamma) * 255.0) as u8;
                let a = (((pix.get_alpha() as f64) / 255.0).powf(self.gamma) * 255.0) as u8;

                Pixels::new(r, g, b, a)
            })
            .collect::<Vec<Pixels>>();

        Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_image.clone(),
        )
    }
}
