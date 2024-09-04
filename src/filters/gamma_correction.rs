use crate::core::{image::Images, operations::Operation, pixel::Pixels};

pub struct GammaCorrection {
    gamma: f64,
}

impl GammaCorrection {
    pub fn new(gamma: f64) -> Self {
        Self { gamma }
    }
}

impl<T> Operation<T> for GammaCorrection
where
    T: Copy + Clone + From<u8> + Into<f64> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let new_image = old_image
            .get_image()
            .iter()
            .map(|pix| {
                let r = (((pix.get_red().into() / 255.0).powf(self.gamma) * 255.0) as u8).into();
                let g = (((pix.get_green().into() / 255.0).powf(self.gamma) * 255.0) as u8).into();
                let b = (((pix.get_blue().into() / 255.0).powf(self.gamma) * 255.0) as u8).into();
                let a = (((pix.get_alpha().into() / 255.0).powf(self.gamma) * 255.0) as u8).into();

                Pixels::new(r, g, b, a)
            })
            .collect::<Vec<Pixels<T>>>();

        Images::new(
            old_image.get_width(),
            old_image.get_height(),
            old_image.get_channels(),
            new_image.clone(),
        )
    }
}
