use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

pub struct FlipVertical {}

impl FlipVertical {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> Operation<T> for FlipVertical
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let original_image = old_image.get_image().clone();

        let new_pixel: Vec<Pixels<T>> = (0..old_image.get_height())
            .into_par_iter()
            .rev()
            .flat_map(|y_index| {
                let mut vec_slice: Vec<Pixels<T>> = Vec::from_iter(
                    original_image[(y_index * old_image.get_width()) as usize
                        ..((y_index * old_image.get_width()) + old_image.get_width()) as usize]
                        .iter()
                        .cloned(),
                );

                vec_slice.reverse();
                vec_slice
            })
            .collect::<Vec<Pixels<T>>>();

        let flipped_image: Images<T> = Images::new(
            old_image.get_width(),
            old_image.get_height(),
            old_image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}

pub struct FlipHorizontal {}

impl FlipHorizontal {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> Operation<T> for FlipHorizontal
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let original_image = old_image.get_image().clone();

        let new_pixel: Vec<Pixels<T>> = (0..old_image.get_height())
            .into_par_iter()
            .flat_map(|y_index| {
                let mut vec_slice: Vec<Pixels<T>> = Vec::from_iter(
                    original_image[(y_index * old_image.get_width()) as usize
                        ..((y_index * old_image.get_width()) + old_image.get_width()) as usize]
                        .iter()
                        .cloned(),
                );

                vec_slice.reverse();
                vec_slice
            })
            .collect::<Vec<Pixels<T>>>();

        let flipped_image: Images<T> = Images::new(
            old_image.get_width(),
            old_image.get_height(),
            old_image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}

pub struct Flip90Left {}

impl Flip90Left {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> Operation<T> for Flip90Left
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let original_image = old_image.get_image().clone();

        let new_pixel: Vec<Pixels<T>> = (0..old_image.get_width())
            .into_par_iter()
            .flat_map(|x_index| {
                let mut vec_slice: Vec<Pixels<T>> = original_image
                    .iter()
                    .skip(x_index as usize)
                    .step_by(old_image.get_height() as usize)
                    .cloned()
                    .collect();

                vec_slice.reverse();
                vec_slice
            })
            .collect::<Vec<Pixels<T>>>();

        let flipped_image: Images<T> = Images::new(
            old_image.get_width(),
            old_image.get_height(),
            old_image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}

pub struct Flip90Right {}

impl Flip90Right {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> Operation<T> for Flip90Right
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let original_image: Vec<Pixels<T>> = old_image.get_image().clone();

        let new_pixel = (0..old_image.get_width())
            .into_par_iter()
            .flat_map(|x_index| {
                let vec_slice: Vec<Pixels<T>> = original_image
                    .iter()
                    .skip(x_index as usize)
                    .step_by(old_image.get_height() as usize)
                    .cloned()
                    .collect();

                vec_slice
            })
            .collect::<Vec<Pixels<T>>>();

        let flipped_image: Images<T> = Images::new(
            old_image.get_width(),
            old_image.get_height(),
            old_image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}
