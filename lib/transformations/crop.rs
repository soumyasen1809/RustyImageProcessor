use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

pub struct Crop {
    top_left_point: (u32, u32),
    new_width: u32,
    new_height: u32,
    image: Images,
}

impl Crop {
    pub fn new(
        top_left_point: (u32, u32),
        new_width: u32,
        new_height: u32,
        image: &Images,
    ) -> Self {
        Self {
            top_left_point,
            new_width,
            new_height,
            image: image.clone(),
        }
    }
}

impl Operation for Crop {
    fn apply(&self) -> Images {
        let mut cropped_image = Images::new(
            self.new_width,
            self.new_height,
            self.image.get_channels(),
            Vec::new(),
        );

        // for y_index in 0..self.new_height as usize {
        //     for x_index in 0..self.new_width as usize {
        //         let pix = self
        //             .image
        //             .get_pixel_at(
        //                 self.top_left_point.0 + x_index as u32,
        //                 self.top_left_point.1 + y_index as u32,
        //             )
        //             .unwrap();

        //         cropped_image.add_pixel(pix);
        //     }
        // }
        let new_image = (0..self.new_height as usize)
            .into_par_iter()
            .flat_map(|y_index| {
                (0..self.new_width as usize)
                    .into_par_iter()
                    .map(|x_index| {
                        let pix = self
                            .image
                            .get_pixel_at(
                                self.top_left_point.0 + x_index as u32,
                                self.top_left_point.1 + y_index as u32,
                            )
                            .unwrap();

                        pix
                    })
                    .collect::<Vec<Pixels>>()
            })
            .collect::<Vec<Pixels>>();

        for pix in new_image.iter() {
            cropped_image.add_pixel(pix.clone());
        }

        cropped_image
    }
}
