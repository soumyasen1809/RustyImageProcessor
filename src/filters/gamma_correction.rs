use crate::core::{image::Images, operations::Operation, pixel::Pixels};

pub struct GammaCorrection<T>
where
    T: Copy + Clone + From<u8> + Into<f64> + std::cmp::PartialEq + Send + Sync,
{
    image: Images<T>,
    gamma: f64,
}

impl<T> GammaCorrection<T>
where
    T: Copy + Clone + From<u8> + Into<f64> + std::cmp::PartialEq + Send + Sync,
{
    pub fn new(image: &Images<T>, gamma: f64) -> Self {
        Self {
            image: image.clone(),
            gamma,
        }
    }
}

impl<T> Operation<T> for GammaCorrection<T>
where
    T: Copy + Clone + From<u8> + Into<f64> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self) -> Images<T> {
        let new_image = self
            .image
            .get_image()
            .iter()
            .map(|pix| {
                let r = (((pix.get_red().into() / 255.0).powf(self.gamma) * 255.0) as u8)
                    .into();
                let g = (((pix.get_green().into() / 255.0).powf(self.gamma) * 255.0)
                    as u8)
                    .into();
                let b = (((pix.get_blue().into() / 255.0).powf(self.gamma) * 255.0) as u8)
                    .into();
                let a = (((pix.get_alpha().into() / 255.0).powf(self.gamma) * 255.0)
                    as u8)
                    .into();

                Pixels::new(r, g, b, a)
            })
            .collect::<Vec<Pixels<T>>>();

        Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_image.clone(),
        )
    }
}
