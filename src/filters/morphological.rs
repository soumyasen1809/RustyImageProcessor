use crate::core::{image::Images, operations::Operation, pixel::Pixels};

#[derive(Clone, Copy)]
pub enum MorphologicalKernelChoices {
    Cross,
    Diamond,
    Horizontal,
    Vertical,
    Diagonal,
    Diagonal2,
}

fn choose_kernel(kernel: MorphologicalKernelChoices) -> Vec<i32> {
    match kernel {
        MorphologicalKernelChoices::Cross => {
            vec![0, 1, 0, 1, 1, 1, 0, 1, 0]
        }
        MorphologicalKernelChoices::Diamond => {
            vec![0, 1, 0, 1, 1, 1, 0, 1, 0]
        }
        MorphologicalKernelChoices::Horizontal => {
            vec![1, 1, 1, 0, 0, 0, 1, 1, 1]
        }
        MorphologicalKernelChoices::Vertical => {
            vec![1, 0, 1, 1, 0, 1, 1, 0, 1]
        }
        MorphologicalKernelChoices::Diagonal => {
            vec![0, 0, 1, 0, 1, 0, 1, 0, 0]
        }
        MorphologicalKernelChoices::Diagonal2 => {
            vec![1, 0, 0, 0, 1, 0, 0, 0, 1]
        }
    }
}

pub struct Erosion {
    kernel_choice: MorphologicalKernelChoices,
}

impl Erosion {
    pub fn new(kernel_choice: MorphologicalKernelChoices) -> Self {
        Self { kernel_choice }
    }
}

impl<T> Operation<T> for Erosion
where
    T: Copy + Clone + From<u8> + From<u8> + Into<u32> + std::cmp::PartialEq + Ord + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let mut new_image: Images<T> = Images::new(
            old_image.get_width(),
            old_image.get_height(),
            old_image.get_channels(),
            Vec::new(),
        );
        let mut image_data: Vec<Pixels<T>> = vec![
            Pixels::default();
            (old_image.get_width() * old_image.get_height())
                .try_into()
                .unwrap()
        ];

        let kernel: Vec<i32> = choose_kernel(self.kernel_choice);

        for y_index in 1..old_image.get_height() - 1 {
            for x_index in 1..old_image.get_width() - 1 {
                let mut min_val: (T, T, T, T) = (255.into(), 255.into(), 255.into(), 255.into());

                for index in 0..kernel.len() {
                    let dx = (index % 3) as i32 - 1;
                    let dy = (index / 3) as i32 - 1;
                    if (x_index as i32) + dx >= 0
                        && (x_index as i32) + dx < old_image.get_width() as i32
                        && (y_index as i32) + dy >= 0
                        && (y_index as i32) + dy < old_image.get_height() as i32
                    {
                        let pix = old_image
                            .get_pixel_at(
                                (x_index as i32 + dx).try_into().unwrap(),
                                (y_index as i32 + dy).try_into().unwrap(),
                            )
                            .unwrap();
                        if *kernel.get(index).unwrap() != 0 {
                            min_val.0 = min_val.0.min(pix.get_red());
                            min_val.1 = min_val.1.min(pix.get_green());
                            min_val.2 = min_val.2.min(pix.get_blue());
                            min_val.3 = min_val.3.min(pix.get_alpha());
                        }
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
    kernel_choice: MorphologicalKernelChoices,
}

impl Dilation {
    pub fn new(kernel_choice: MorphologicalKernelChoices) -> Self {
        Self { kernel_choice }
    }
}

impl<T> Operation<T> for Dilation
where
    T: Copy + Clone + From<u8> + From<u8> + Into<u32> + std::cmp::PartialEq + Ord + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let mut new_image: Images<T> = Images::new(
            old_image.get_width(),
            old_image.get_height(),
            old_image.get_channels(),
            Vec::new(),
        );
        let mut image_data: Vec<Pixels<T>> = vec![
            Pixels::default();
            (old_image.get_width() * old_image.get_height())
                .try_into()
                .unwrap()
        ];

        let kernel: Vec<i32> = choose_kernel(self.kernel_choice);

        for y_index in 1..old_image.get_height() - 1 {
            for x_index in 1..old_image.get_width() - 1 {
                let mut max_val: (T, T, T, T) = (0.into(), 0.into(), 0.into(), 0.into());

                for index in 0..kernel.len() {
                    let dx = (index % 3) as i32 - 1;
                    let dy = (index / 3) as i32 - 1;
                    if (x_index as i32) + dx >= 0
                        && (x_index as i32) + dx < old_image.get_width() as i32
                        && (y_index as i32) + dy >= 0
                        && (y_index as i32) + dy < old_image.get_height() as i32
                    {
                        let pix = old_image
                            .get_pixel_at(
                                (x_index as i32 + dx).try_into().unwrap(),
                                (y_index as i32 + dy).try_into().unwrap(),
                            )
                            .unwrap();
                        if *kernel.get(index).unwrap() != 0 {
                            max_val.0 = max_val.0.max(pix.get_red());
                            max_val.1 = max_val.1.max(pix.get_green());
                            max_val.2 = max_val.2.max(pix.get_blue());
                            max_val.3 = max_val.3.max(pix.get_alpha());
                        }
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
