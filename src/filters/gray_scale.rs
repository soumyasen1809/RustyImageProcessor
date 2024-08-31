use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

#[derive(Clone, Copy)]
pub enum GrayScaleAlgorithms {
    Average,
    Luminosity,
}

fn select_grayscale_algorithm<T>(algo: &GrayScaleAlgorithms, pix: &Pixels<T>) -> T
where
    T: Copy + Clone + From<u8> + From<f64> + Into<u32> + std::cmp::PartialEq,
{
    match algo {
        GrayScaleAlgorithms::Average => (((pix.get_red().into() as f64
            + pix.get_green().into() as f64
            + pix.get_blue().into() as f64)
            / 3.0) as u8)
            .into(),

        GrayScaleAlgorithms::Luminosity => {
            // Luminosity method: https://www.mathworks.com/help/matlab/ref/rgb2gray.html
            ((((pix.get_red().into() as f64 * 0.299)
                + (pix.get_green().into() as f64 * 0.5879)
                + (pix.get_blue().into() as f64 * 0.114))
                / 3.0) as u8)
                .into()
        }
    }
}

pub struct GrayScale<T>
where
    T: Copy + Clone + From<u8> + From<f64> + Into<u32> + std::cmp::PartialEq + Send + Sync,
{
    image: Images<T>,
    algo: GrayScaleAlgorithms,
}

impl<T> GrayScale<T>
where
    T: Copy + Clone + From<u8> + From<f64> + Into<u32> + std::cmp::PartialEq + Send + Sync,
{
    pub fn new(image: &Images<T>, algo: GrayScaleAlgorithms) -> Self {
        Self {
            image: image.clone(),
            algo,
        }
    }
}

impl<T> Operation<T> for GrayScale<T>
where
    T: Copy + Clone + From<u8> + From<f64> + Into<u32> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self) -> Images<T> {
        let new_pixel = self
            .image
            .get_image()
            .into_par_iter()
            .map(|pix| {
                let grayscale_pixel = select_grayscale_algorithm(&self.algo, &pix);
                Pixels::new(
                    grayscale_pixel,
                    grayscale_pixel,
                    grayscale_pixel,
                    pix.get_alpha(),
                )
            })
            .collect::<Vec<Pixels<T>>>();

        Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_pixel.clone(),
        )
    }
}
