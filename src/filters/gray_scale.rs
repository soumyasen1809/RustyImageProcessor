use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

#[derive(Clone, Copy)]
pub enum GrayScaleAlgorithms {
    Average,
    Luminosity,
}

fn select_grayscale_algorithm(algo: &GrayScaleAlgorithms, pix: &Pixels) -> u8 {
    match algo {
        GrayScaleAlgorithms::Average => {
            ((pix.get_red() as f64 + pix.get_green() as f64 + pix.get_blue() as f64) / 3.0) as u8
        }

        GrayScaleAlgorithms::Luminosity => {
            // Luminosity method: https://www.mathworks.com/help/matlab/ref/rgb2gray.html
            (((pix.get_red() as f64 * 0.299)
                + (pix.get_green() as f64 * 0.5879)
                + (pix.get_blue() as f64 * 0.114))
                / 3.0) as u8
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
        let new_pixel: Vec<Pixels> = self
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
            .collect::<Vec<Pixels>>();

        let new_image = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_pixel.clone(),
        );

        return new_image;
    }
}
