use crate::core::{image::Images, pixel::Pixels};

pub trait Transformation {
    fn apply(&self) -> Images;
}

pub enum TransformationOperations {
    FLIPVERTICAL,
    FLIPHORIZONTAL,
    FLIP90LEFT,
    FLIP90RIGHT,
}

impl TransformationOperations {
    pub fn chain_operations(image: &Images, operations: Vec<TransformationOperations>) -> Images {
        let mut new_image: Images = image.clone();

        for ops in operations.iter() {
            new_image = match ops {
                TransformationOperations::FLIPVERTICAL => FlipVertical::new(&new_image).apply(),
                TransformationOperations::FLIPHORIZONTAL => FlipHorizontal::new(&new_image).apply(),
                TransformationOperations::FLIP90LEFT => Flip90Left::new(&new_image).apply(),
                TransformationOperations::FLIP90RIGHT => Flip90Right::new(&new_image).apply(),
            };
        }

        new_image
    }
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

impl Transformation for FlipVertical {
    fn apply(&self) -> Images {
        let original_image = self.image.get_image().clone();

        let mut flipped_image: Images = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            Vec::new(),
        );

        for y_index in (0..self.image.get_height()).rev() {
            let mut vec_slice: Vec<Pixels> = Vec::from_iter(
                original_image[(y_index * self.image.get_width()) as usize
                    ..((y_index * self.image.get_width()) + self.image.get_width()) as usize]
                    .iter()
                    .cloned(),
            );

            vec_slice.reverse();
            for pix in vec_slice.iter() {
                flipped_image.add_pixel(pix.clone());
            }
        }

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

impl Transformation for FlipHorizontal {
    fn apply(&self) -> Images {
        let original_image = self.image.get_image().clone();

        let mut flipped_image: Images = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            Vec::new(),
        );

        for y_index in 0..self.image.get_height() {
            let mut vec_slice: Vec<Pixels> = Vec::from_iter(
                original_image[(y_index * self.image.get_width()) as usize
                    ..((y_index * self.image.get_width()) + self.image.get_width()) as usize]
                    .iter()
                    .cloned(),
            );

            vec_slice.reverse();
            for pix in vec_slice.iter() {
                flipped_image.add_pixel(pix.clone());
            }
        }

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

impl Transformation for Flip90Left {
    fn apply(&self) -> Images {
        let original_image = self.image.get_image().clone();

        let mut flipped_image: Images = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            Vec::new(),
        );

        for x_index in 0..self.image.get_width() {
            let mut vec_slice: Vec<Pixels> = original_image
                .iter()
                .skip(x_index as usize)
                .step_by(self.image.get_height() as usize)
                .cloned()
                .collect();

            vec_slice.reverse();
            for pix in vec_slice.iter() {
                flipped_image.add_pixel(pix.clone());
            }
        }

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

impl Transformation for Flip90Right {
    fn apply(&self) -> Images {
        let original_image = self.image.get_image().clone();

        let mut flipped_image: Images = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            Vec::new(),
        );

        for x_index in 0..self.image.get_width() {
            let vec_slice: Vec<Pixels> = original_image
                .iter()
                .skip(x_index as usize)
                .step_by(self.image.get_height() as usize)
                .cloned()
                .collect();

            for pix in vec_slice.iter() {
                flipped_image.add_pixel(pix.clone());
            }
        }

        flipped_image
    }
}
