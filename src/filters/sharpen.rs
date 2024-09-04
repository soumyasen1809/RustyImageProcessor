use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

#[derive(Clone, Copy)]
pub enum SharpeningKernelChoices {
    Basic,
    HighPass,
    EdgeEnhancement,
}

fn select_smoothing_kernel(choice: SharpeningKernelChoices) -> Vec<i32> {
    match choice {
        SharpeningKernelChoices::Basic => vec![0, -1, 0, -1, 5, -1, 0, -1, 0],
        SharpeningKernelChoices::HighPass => vec![-1, -1, -1, -1, 8, -1, -1, -1, -1],
        SharpeningKernelChoices::EdgeEnhancement => vec![-1, -1, -1, -1, 9, -1, -1, -1, -1],
    }
}

pub struct Sharpen {
    kernel_choice: SharpeningKernelChoices,
}

impl Sharpen {
    pub fn new(kernel_choice: SharpeningKernelChoices) -> Self {
        Self { kernel_choice }
    }
}

impl<T> Operation<T> for Sharpen
where
    T: Copy + Clone + From<u8> + Into<u32> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let kernel: Vec<i32> = select_smoothing_kernel(self.kernel_choice);

        let kernel_size: u32 = 3;
        let half_kernel_size = kernel_size / 2;

        let output_width = old_image.get_width() - 2 * half_kernel_size;
        let output_height = old_image.get_height() - 2 * half_kernel_size;

        let new_pixel = (half_kernel_size..old_image.get_height() - half_kernel_size)
            .into_par_iter()
            .flat_map(|y_index| {
                (half_kernel_size..old_image.get_width() - half_kernel_size)
                    .into_par_iter()
                    .map(|x_index| {
                        let sum_vec: Vec<(i32, i32, i32)> = (0..kernel_size)
                            .into_par_iter()
                            .flat_map(|dy| {
                                (0..kernel_size)
                                    .into_par_iter()
                                    .map(|dx| {
                                        let pixel = old_image
                                            .get_pixel_at(
                                                x_index + dx - half_kernel_size,
                                                y_index + dy - half_kernel_size,
                                            )
                                            .unwrap();
                                        let kernel_val =
                                            *kernel.get((dy * kernel_size + dx) as usize).unwrap();
                                        (
                                            (pixel.get_red().into() as i32) * (kernel_val),
                                            (pixel.get_green().into() as i32) * (kernel_val),
                                            (pixel.get_blue().into() as i32) * (kernel_val),
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

                        Pixels::new(
                            (sum_r.clamp(0, 255) as u8).into(),
                            (sum_g.clamp(0, 255) as u8).into(),
                            (sum_b.clamp(0, 255) as u8).into(),
                            255_u8.into(),
                        )
                    })
                    .collect::<Vec<Pixels<T>>>()
            })
            .collect::<Vec<Pixels<T>>>();

        Images::new(
            output_width,
            output_height,
            old_image.get_channels(),
            new_pixel.clone(),
        )
    }
}
