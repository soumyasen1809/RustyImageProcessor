use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

pub struct Crop {
    top_left_point: (u32, u32),
    new_width: u32,
    new_height: u32,
}

impl Crop {
    pub fn new(top_left_point: (u32, u32), new_width: u32, new_height: u32) -> Self {
        Self {
            top_left_point,
            new_width,
            new_height,
        }
    }
}

impl<T> Operation<T> for Crop
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let new_pixel = (0..self.new_height as usize)
            .into_par_iter()
            .flat_map(|y_index| {
                (0..self.new_width as usize)
                    .into_par_iter()
                    .map(|x_index| {
                        old_image
                            .get_pixel_at(
                                self.top_left_point.0 + x_index as u32,
                                self.top_left_point.1 + y_index as u32,
                            )
                            .unwrap()
                    })
                    .collect::<Vec<Pixels<T>>>()
            })
            .collect::<Vec<Pixels<T>>>();

        Images::new(
            self.new_width,
            self.new_height,
            old_image.get_channels(),
            new_pixel.clone(),
        )
    }
}
