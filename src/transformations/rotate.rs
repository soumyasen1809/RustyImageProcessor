use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

pub enum RotatingOperations {
    RotateVertical,
    RotateHorizontal,
    Rotate90Left,
    Rotate90Right,
}

pub struct FlipVertical {
    image: Images,
}

impl FlipVertical {
    pub fn new(image: &Images) -> Self {
        Self {
            image: image.clone(),
        }
    }
}

impl Operation for FlipVertical {
    fn apply(&self) -> Images {
        let original_image = self.image.get_image().clone();

        let new_pixel = (0..self.image.get_height())
            .into_par_iter()
            .rev()
            .flat_map(|y_index| {
                let mut vec_slice: Vec<Pixels> = Vec::from_iter(
                    original_image[(y_index * self.image.get_width()) as usize
                        ..((y_index * self.image.get_width()) + self.image.get_width()) as usize]
                        .iter()
                        .cloned(),
                );

                vec_slice.reverse();
                vec_slice
            })
            .collect::<Vec<Pixels>>();

        let flipped_image: Images = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}

pub struct FlipHorizontal {
    image: Images,
}

impl FlipHorizontal {
    pub fn new(image: &Images) -> Self {
        Self {
            image: image.clone(),
        }
    }
}

impl Operation for FlipHorizontal {
    fn apply(&self) -> Images {
        let original_image = self.image.get_image().clone();

        let new_pixel = (0..self.image.get_height())
            .into_par_iter()
            .flat_map(|y_index| {
                let mut vec_slice: Vec<Pixels> = Vec::from_iter(
                    original_image[(y_index * self.image.get_width()) as usize
                        ..((y_index * self.image.get_width()) + self.image.get_width()) as usize]
                        .iter()
                        .cloned(),
                );

                vec_slice.reverse();
                vec_slice
            })
            .collect::<Vec<Pixels>>();

        let flipped_image: Images = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}

pub struct Flip90Left {
    image: Images,
}

impl Flip90Left {
    pub fn new(image: &Images) -> Self {
        Self {
            image: image.clone(),
        }
    }
}

impl Operation for Flip90Left {
    fn apply(&self) -> Images {
        let original_image = self.image.get_image().clone();

        let new_pixel = (0..self.image.get_width())
            .into_par_iter()
            .flat_map(|x_index| {
                let mut vec_slice: Vec<Pixels> = original_image
                    .iter()
                    .skip(x_index as usize)
                    .step_by(self.image.get_height() as usize)
                    .cloned()
                    .collect();

                vec_slice.reverse();
                vec_slice
            })
            .collect::<Vec<Pixels>>();

        let flipped_image: Images = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}

pub struct Flip90Right {
    image: Images,
}

impl Flip90Right {
    pub fn new(image: &Images) -> Self {
        Self {
            image: image.clone(),
        }
    }
}

impl Operation for Flip90Right {
    fn apply(&self) -> Images {
        let original_image = self.image.get_image().clone();

        let new_pixel = (0..self.image.get_width())
            .into_par_iter()
            .flat_map(|x_index| {
                let vec_slice: Vec<Pixels> = original_image
                    .iter()
                    .skip(x_index as usize)
                    .step_by(self.image.get_height() as usize)
                    .cloned()
                    .collect();

                vec_slice
            })
            .collect::<Vec<Pixels>>();

        let flipped_image: Images = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            new_pixel.clone(),
        );

        flipped_image
    }
}
