use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

#[derive(Clone, Copy)]
pub enum GrayScaleAlgorithms {
    Average,
    Luminosity,
}

fn select_grayscale_algorithm(algo: &GrayScaleAlgorithms, pix: &Pixels) -> u8 {
    match algo {
        // average grayscale algorithm
        GrayScaleAlgorithms::Average => (pix.get_red() + pix.get_green() + pix.get_blue()) / 3,

        GrayScaleAlgorithms::Luminosity => {
            // Luminosity method: https://www.mathworks.com/help/matlab/ref/rgb2gray.html
            (((pix.get_red() as f64 * 0.299) as u8)
                + ((pix.get_green() as f64 * 0.5879) as u8)
                + (pix.get_blue() as f64 * 0.114) as u8)
                / 3
        }
    }
}

pub struct GrayScale {
    image: Images,
    algo: GrayScaleAlgorithms,
}

impl GrayScale {
    pub fn new(image: &Images, algo: GrayScaleAlgorithms) -> Self {
        Self {
            image: image.clone(),
            algo,
        }
    }
}

impl Operation for GrayScale {
    fn apply(&self) -> Images {
        let grayscale_pixel: Vec<u8> = self
            .image
            .get_image()
            .iter()
            .map(|pix| select_grayscale_algorithm(&self.algo, pix))
            .collect();

        let pixels: Vec<Pixels> = (0..self.image.get_image().len())
            .into_par_iter()
            .map(|index| {
                Pixels::new(
                    *grayscale_pixel.get(index).unwrap(),
                    *grayscale_pixel.get(index).unwrap(),
                    *grayscale_pixel.get(index).unwrap(),
                    (*self.image.get_image().get(index).unwrap()).get_alpha(),
                )
            })
            .collect();

        let new_image = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            pixels.clone(),
        );

        return new_image;
    }
}
