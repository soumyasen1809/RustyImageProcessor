use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

#[derive(Clone, Copy)]
pub enum EdgeDetectingKernelChoices {
    Outline,
    SobelX,
    SobelY,
    Emboss,
}

fn select_edge_detecting_kernel(choice: EdgeDetectingKernelChoices) -> Vec<i32> {
    match choice {
        EdgeDetectingKernelChoices::Outline => vec![-1, -1, -1, -1, 8, -1, -1, -1, -1],
        EdgeDetectingKernelChoices::SobelX => vec![-1, 0, 1, -2, 0, 2, -1, 0, 1],
        EdgeDetectingKernelChoices::SobelY => vec![-1, -2, -1, 0, 0, 0, 1, 2, 1],
        EdgeDetectingKernelChoices::Emboss => vec![-2, -1, 0, -1, 1, 1, 0, 2, 2],
    }
}

pub struct EdgeDetection {
    image: Images,
    kernel_choice: EdgeDetectingKernelChoices,
}

impl EdgeDetection {
    pub fn new(image: &Images, kernel_choice: EdgeDetectingKernelChoices) -> Self {
        Self {
            image: image.clone(),
            kernel_choice,
        }
    }
}

impl Operation for EdgeDetection {
    fn apply(&self) -> Images {
        let kernel: Vec<i32> = select_edge_detecting_kernel(self.kernel_choice);

        let kernel_size: u32 = 3;
        let half_kernel_size = kernel_size / 2;

        let output_width = self.image.get_width() - 2 * half_kernel_size;
        let output_height = self.image.get_height() - 2 * half_kernel_size;

        let new_pixel = (half_kernel_size..self.image.get_height() - half_kernel_size)
            .into_par_iter()
            .flat_map(|y_index| {
                (half_kernel_size..self.image.get_width() - half_kernel_size)
                    .into_par_iter()
                    .map(|x_index| {
                        let sum_vec: Vec<(i32, i32, i32)> = (0..kernel_size)
                            .into_par_iter()
                            .flat_map(|dy| {
                                (0..kernel_size)
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
                                            (pixel.get_red() as i32) * (kernel_val),
                                            (pixel.get_green() as i32) * (kernel_val),
                                            (pixel.get_blue() as i32) * (kernel_val),
                                        )
                                    })
                                    .collect::<Vec<(i32, i32, i32)>>()
                            })
                            .collect::<Vec<(i32, i32, i32)>>();

                        // Intermediate calculations using sum_r, sum_g, and sum_b to provide more accurate results
                        let mut sum_r = 0;
                        let mut sum_g = 0;
                        let mut sum_b = 0;
                        for sum in sum_vec.iter() {
                            sum_r += sum.0;
                            sum_g += sum.1;
                            sum_b += sum.2;
                        }

                        // For sharpening, the kernel is designed to highlight edges and details.
                        // The sum of the elements in a sharpening kernel is usually zero or close to zero,
                        // which means normalizing by the sum would lead to incorrect results.
                        let new_pixel = Pixels::new(
                            sum_r.clamp(0, 255) as u8,
                            sum_g.clamp(0, 255) as u8,
                            sum_b.clamp(0, 255) as u8,
                            255,
                        );

                        new_pixel
                    })
                    .collect::<Vec<Pixels>>()
            })
            .collect::<Vec<Pixels>>();

        let output_image = Images::new(
            output_width,
            output_height,
            self.image.get_channels(),
            new_pixel.clone(),
        );

        output_image
    }
}
