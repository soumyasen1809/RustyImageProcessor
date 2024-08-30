use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

#[derive(Clone, Copy)]
pub enum SmoothingKernelChoices {
    Gaussian,
    BoxBlur,
}

fn select_smoothing_kernel(choice: SmoothingKernelChoices) -> Vec<u8> {
    match choice {
        SmoothingKernelChoices::Gaussian => vec![1, 2, 1, 2, 4, 2, 1, 2, 1], // Gaussian blur kernel for better smoothing
        SmoothingKernelChoices::BoxBlur => vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
    }
}

pub struct Blur {
    image: Images,
    kernel_choice: SmoothingKernelChoices,
}

impl Blur {
    pub fn new(image: &Images, kernel_choice: SmoothingKernelChoices) -> Self {
        Self {
            image: image.clone(),
            kernel_choice,
        }
    }
}

// AI: Algorithm from Gemini
impl Operation for Blur {
    fn apply(&self) -> Images {
        let kernel = select_smoothing_kernel(self.kernel_choice);
        let kernel_size: u32 = 3;
        let kernel_normalizer: u32 = kernel.iter().map(|x| *x as u32).sum(); // Kernel sum normalization gives darker images
                                                                             // let kernel_normalizer: u32 = *kernel.iter().max().unwrap() as u32;
        let half_kernel_size = kernel_size / 2;

        let output_width = self.image.get_width() - 2 * half_kernel_size;
        let output_height = self.image.get_height() - 2 * half_kernel_size;

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
                                                x_index + dx - half_kernel_size,
                                                y_index + dy - half_kernel_size,
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

                        

                        Pixels::new(
                            (sum_r / kernel_normalizer).clamp(0, 255) as u8,
                            (sum_g / kernel_normalizer).clamp(0, 255) as u8,
                            (sum_b / kernel_normalizer).clamp(0, 255) as u8,
                            255,
                        )
                    })
                    .collect::<Vec<Pixels>>()
            })
            .collect::<Vec<Pixels>>();

        

        Images::new(
            output_width,
            output_height,
            self.image.get_channels(),
            new_pixel.clone(),
        )
    }
}
