use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

pub enum RotatingOperations {
    RotateVertical,
    RotateHorizontal,
    Rotate90Left,
    Rotate90Right,
}

pub struct FlipVertical<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    image: Images<T>,
}

impl<T> FlipVertical<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    pub fn new(image: &Images<T>) -> Self {
        Self {
            image: image.clone(),
        }
    }
}

impl<T> Operation<T> for FlipVertical<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self) -> Images<T> {
        let original_image = self.image.get_image().clone();

        let new_pixel: Vec<Pixels<T>> = (0..self.image.get_height())
            .into_par_iter()
            .rev()
            .flat_map(|y_index| {
                let mut vec_slice: Vec<Pixels<T>> = Vec::from_iter(
                    original_image[(y_index * self.image.get_width()) as usize
                        ..((y_index * self.image.get_width()) + self.image.get_width()) as usize]
                        .iter()
                        .cloned(),
                );

                vec_slice.reverse();
                vec_slice
            })
            .collect::<Vec<Pixels<T>>>();

        let flipped_image: Images<T> = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}

pub struct FlipHorizontal<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    image: Images<T>,
}

impl<T> FlipHorizontal<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    pub fn new(image: &Images<T>) -> Self {
        Self {
            image: image.clone(),
        }
    }
}

impl<T> Operation<T> for FlipHorizontal<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self) -> Images<T> {
        let original_image = self.image.get_image().clone();

        let new_pixel: Vec<Pixels<T>> = (0..self.image.get_height())
            .into_par_iter()
            .flat_map(|y_index| {
                let mut vec_slice: Vec<Pixels<T>> = Vec::from_iter(
                    original_image[(y_index * self.image.get_width()) as usize
                        ..((y_index * self.image.get_width()) + self.image.get_width()) as usize]
                        .iter()
                        .cloned(),
                );

                vec_slice.reverse();
                vec_slice
            })
            .collect::<Vec<Pixels<T>>>();

        let flipped_image: Images<T> = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}

pub struct Flip90Left<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    image: Images<T>,
}

impl<T> Flip90Left<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    pub fn new(image: &Images<T>) -> Self {
        Self {
            image: image.clone(),
        }
    }
}

impl<T> Operation<T> for Flip90Left<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self) -> Images<T> {
        let original_image = self.image.get_image().clone();

        let new_pixel: Vec<Pixels<T>> = (0..self.image.get_width())
            .into_par_iter()
            .flat_map(|x_index| {
                let mut vec_slice: Vec<Pixels<T>> = original_image
                    .iter()
                    .skip(x_index as usize)
                    .step_by(self.image.get_height() as usize)
                    .cloned()
                    .collect();

                vec_slice.reverse();
                vec_slice
            })
            .collect::<Vec<Pixels<T>>>();

        let flipped_image: Images<T> = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}

pub struct Flip90Right<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    image: Images<T>,
}

impl<T> Flip90Right<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    pub fn new(image: &Images<T>) -> Self {
        Self {
            image: image.clone(),
        }
    }
}

impl<T> Operation<T> for Flip90Right<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self) -> Images<T> {
        let original_image: Vec<Pixels<T>> = self.image.get_image().clone();

        let new_pixel = (0..self.image.get_width())
            .into_par_iter()
            .flat_map(|x_index| {
                let vec_slice: Vec<Pixels<T>> = original_image
                    .iter()
                    .skip(x_index as usize)
                    .step_by(self.image.get_height() as usize)
                    .cloned()
                    .collect();

                vec_slice
            })
            .collect::<Vec<Pixels<T>>>();

        let flipped_image: Images<T> = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}
