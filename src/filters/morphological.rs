use crate::core::{image::Images, operations::Operation, pixel::Pixels};

#[derive(Clone, Copy)]
pub enum MorphologicalKernelChoices {
    CrossKernel,
    DiamondKernel,
    HorizontalKernel,
    VerticalKernel,
    DiagonalKernel,
    DiagonalKernel2,
}

fn choose_kernel(kernel: MorphologicalKernelChoices) -> Vec<i32> {
    match kernel {
        MorphologicalKernelChoices::CrossKernel => {
            vec![0, 1, 0, 1, 1, 1, 0, 1, 0]
        }
        MorphologicalKernelChoices::DiamondKernel => {
            vec![0, 1, 0, 1, 1, 1, 0, 1, 0]
        }
        MorphologicalKernelChoices::HorizontalKernel => {
            vec![1, 1, 1, 0, 0, 0, 1, 1, 1]
        }
        MorphologicalKernelChoices::VerticalKernel => {
            vec![1, 0, 1, 1, 0, 1, 1, 0, 1]
        }
        MorphologicalKernelChoices::DiagonalKernel => {
            vec![0, 0, 1, 0, 1, 0, 1, 0, 0]
        }
        MorphologicalKernelChoices::DiagonalKernel2 => {
            vec![1, 0, 0, 0, 1, 0, 0, 0, 1]
        }
    }
}

pub struct Erosion {
    image: Images,
    kernel_choice: MorphologicalKernelChoices,
}

impl Erosion {
    pub fn new(image: &Images, kernel_choice: MorphologicalKernelChoices) -> Self {
        Self {
            image: image.clone(),
            kernel_choice,
        }
    }
}

impl Operation for Erosion {
    fn apply(&self) -> Images {
        let mut new_image = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            Vec::new(),
        );
        let mut image_data = vec![
            Pixels::default();
            (self.image.get_width() * self.image.get_height())
                .try_into()
                .unwrap()
        ];

        let kernel: Vec<i32> = choose_kernel(self.kernel_choice);

        for y_index in 1..self.image.get_height() - 1 {
            for x_index in 1..self.image.get_width() - 1 {
                let mut min_val: (u8, u8, u8, u8) = (255, 255, 255, 255);

                for index in 0..kernel.len() {
                    let dx = index % 3 - 1;
                    let dy = index / 3 - 1;
                    let pix = self
                        .image
                        .get_pixel_at(x_index + dx as u32, y_index + dy as u32)
                        .unwrap();
                    if *kernel.get(index).unwrap() != 0 {
                        min_val.0 = min_val.0.min(pix.get_red());
                        min_val.1 = min_val.1.min(pix.get_green());
                        min_val.2 = min_val.2.min(pix.get_blue());
                        min_val.3 = min_val.3.min(pix.get_alpha());
                    }
                }

                image_data[((y_index - 1) * new_image.get_width() + (x_index - 1)) as usize] =
                    Pixels::new(min_val.0, min_val.1, min_val.2, min_val.3);
            }
        }

        for pix in image_data.iter() {
            new_image.add_pixel(pix.clone());
        }

        new_image
    }
}

pub struct Dilation {
    image: Images,
    kernel_choice: MorphologicalKernelChoices,
}

impl Dilation {
    pub fn new(image: &Images, kernel_choice: MorphologicalKernelChoices) -> Self {
        Self {
            image: image.clone(),
            kernel_choice,
        }
    }
}

impl Operation for Dilation {
    fn apply(&self) -> Images {
        let mut new_image = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            Vec::new(),
        );
        let mut image_data = vec![
            Pixels::default();
            (self.image.get_width() * self.image.get_height())
                .try_into()
                .unwrap()
        ];

        let kernel: Vec<i32> = choose_kernel(self.kernel_choice);

        for y_index in 1..self.image.get_height() - 1 {
            for x_index in 1..self.image.get_width() - 1 {
                let mut max_val: (u8, u8, u8, u8) = (0, 0, 0, 0);

                for index in 0..kernel.len() {
                    let dx = index % 3 - 1;
                    let dy = index / 3 - 1;
                    let pix = self
                        .image
                        .get_pixel_at(x_index + dx as u32, y_index + dy as u32)
                        .unwrap();
                    if *kernel.get(index).unwrap() != 0 {
                        max_val.0 = max_val.0.max(pix.get_red());
                        max_val.1 = max_val.1.max(pix.get_green());
                        max_val.2 = max_val.2.max(pix.get_blue());
                        max_val.3 = max_val.3.max(pix.get_alpha());
                    }
                }

                image_data[((y_index - 1) * new_image.get_width() + (x_index - 1)) as usize] =
                    Pixels::new(max_val.0, max_val.1, max_val.2, max_val.3);
            }
        }

        for pix in image_data.iter() {
            new_image.add_pixel(pix.clone());
        }

        new_image
    }
}
