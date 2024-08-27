use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    core::{image::Images, pixel::Pixels},
    transformations::rotate::Transformation,
};

#[derive(Clone, Copy)]
pub enum SmoothingKernelChoice {
    GAUSSIAN,
    BOXBLUR,
}

pub struct Blur {
    image: Images,
    kernel_choice: SmoothingKernelChoice,
}

impl Blur {
    pub fn new(image: &Images, kernel_choice: SmoothingKernelChoice) -> Self {
        Self {
            image: image.clone(),
            kernel_choice,
        }
    }
}

fn select_smoothing_kernel(choice: SmoothingKernelChoice) -> Vec<u8> {
    match choice {
        SmoothingKernelChoice::GAUSSIAN => vec![1, 2, 1, 2, 4, 2, 1, 2, 1], // Gaussian blur kernel for better smoothing
        SmoothingKernelChoice::BOXBLUR => vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
    }
}

// AI: Algorithm from Gemini
impl Transformation for Blur {
    fn apply(&self) -> Images {
        let kernel = select_smoothing_kernel(self.kernel_choice);
        let kernel_size: u32 = 3;
        // let kernel_sum: u32 = kernel.iter().map(|x| *x as u32).sum();    // Kernel sum normalization gives darker images
        let kernel_max: u32 = *kernel.iter().max().unwrap() as u32;
        let half_kernel_size = kernel_size / 2;

        let output_width = self.image.get_width() - 2 * half_kernel_size;
        let output_height = self.image.get_height() - 2 * half_kernel_size;

        let mut output_image = Images::new(
            output_width,
            output_height,
            self.image.get_channels(),
            Vec::new(),
        );

        let new_pixel = (half_kernel_size..self.image.get_height() - half_kernel_size)
            .into_par_iter()
            .flat_map(|y_index| {
                (half_kernel_size..self.image.get_width() - half_kernel_size)
                    .into_par_iter()
                    .map(|x_index| {
                        let sum_vec: Vec<(u32, u32, u32)> = (0..=half_kernel_size)
                            .into_par_iter()
                            .flat_map(|dy| {
                                (0..=half_kernel_size)
                                    .into_par_iter()
                                    .map(|dx| {
                                        let pixel = self
                                            .image
                                            .get_pixel_at(
                                                x_index as u32 + dx - half_kernel_size,
                                                y_index as u32 + dy - half_kernel_size,
                                            )
                                            .unwrap();
                                        let kernel_val =
                                            *kernel.get((dy * kernel_size + dx) as usize).unwrap();
                                        (
                                            (pixel.get_red() as u32) * (kernel_val as u32),
                                            (pixel.get_green() as u32) * (kernel_val as u32),
                                            (pixel.get_blue() as u32) * (kernel_val as u32),
                                        )
                                    })
                                    .collect::<Vec<(u32, u32, u32)>>()
                            })
                            .collect::<Vec<(u32, u32, u32)>>();

                        // Intermediate calculations using sum_r, sum_g, and sum_b to provide more accurate results
                        let mut sum_r = 0;
                        let mut sum_g = 0;
                        let mut sum_b = 0;
                        for sum in sum_vec.iter() {
                            sum_r += sum.0;
                            sum_g += sum.1;
                            sum_b += sum.2;
                        }

                        let new_pixel = Pixels::new(
                            (sum_r / kernel_max) as u8,
                            (sum_g / kernel_max) as u8,
                            (sum_b / kernel_max) as u8,
                            255,
                        );

                        new_pixel
                    })
                    .collect::<Vec<Pixels>>()
            })
            .collect::<Vec<Pixels>>();

        for pix in new_pixel.iter() {
            output_image.add_pixel(pix.clone());
        }

        output_image
    }
}
