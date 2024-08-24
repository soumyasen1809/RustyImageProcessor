use crate::core::{image::Images, pixel::Pixels};

pub trait Transformation {
    fn apply(&self) -> Images;
}

// pub struct FlipVertical {
//     image: Images,
// }

// impl FlipVertical {
//     pub fn new(image: &Images) -> Self {
//         Self { image: image.clone() }
//     }
// }

// impl Transformation for FlipVertical {
//     fn apply(&self) -> Images {}
// }

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
        let image = self.image.get_image().clone();

        let mut flipped_image: Images = Images::new(
            self.image.get_width(),
            self.image.get_height(),
            self.image.get_channels(),
            Vec::new(),
        );

        for y_index in 0..self.image.get_height() {
            let mut vec_slice: Vec<Pixels> = Vec::from_iter(
                image[(y_index * self.image.get_width()) as usize
                    ..((y_index * self.image.get_width()) + self.image.get_width()) as usize]
                    .iter()
                    .cloned(),
            );

            vec_slice.reverse();
            for pix in vec_slice.iter() {
                flipped_image.add_data(pix.clone());
            }
        }

        flipped_image
    }
}
