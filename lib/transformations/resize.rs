use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

pub enum ResizingOperations {
    NearestNeighbours(u32, u32),
    BilinearInterpolation(u32, u32),
}

pub struct ResizeNearestNeighbour {
    new_width: u32,
    new_height: u32,
    image: Images,
}

impl ResizeNearestNeighbour {
    pub fn new(new_width: u32, new_height: u32, image: &Images) -> Self {
        Self {
            new_width,
            new_height,
            image: image.clone(),
        }
    }
}

impl Operation for ResizeNearestNeighbour {
    fn apply(&self) -> Images {
        let mut new_image = Images::new(
            self.new_width,
            self.new_height,
            self.image.get_channels(),
            Vec::new(),
        );

        let x_ratio = self.image.get_width() as f64 / self.new_width as f64;
        let y_ratio = self.image.get_height() as f64 / self.new_height as f64;

        let pixel_list = (0..self.new_height)
            .into_par_iter()
            .flat_map(|y_index| {
                (0..self.new_width)
                    .into_par_iter()
                    .map(move |x_index| {
                        self.image
                            .get_pixel_at(
                                (x_index as f64 * x_ratio) as u32,
                                (y_index as f64 * y_ratio) as u32,
                            )
                            .unwrap()
                    })
                    .collect::<Vec<Pixels>>()
            })
            .collect::<Vec<Pixels>>();

        for pix in pixel_list.iter() {
            new_image.add_pixel(pix.clone());
        }

        new_image
    }
}

pub struct ResizeBilinearInterpolation {
    new_width: u32,
    new_height: u32,
    image: Images,
}

impl ResizeBilinearInterpolation {
    pub fn new(new_width: u32, new_height: u32, image: &Images) -> Self {
        Self {
            new_width,
            new_height,
            image: image.clone(),
        }
    }
}

impl Operation for ResizeBilinearInterpolation {
    fn apply(&self) -> Images {
        let mut new_image = Images::new(
            self.new_width,
            self.new_height,
            self.image.get_channels(),
            Vec::new(),
        );

        let x_ratio = self.image.get_width() as f64 / self.new_width as f64;
        let y_ratio = self.image.get_height() as f64 / self.new_height as f64;

        let pixel_list = (0..self.new_height)
            .into_par_iter()
            .flat_map(|y_index| {
                (0..self.new_width)
                    .into_par_iter()
                    .map(|x_index| {
                        let x1 = (x_index as f64 * x_ratio) as u32;
                        let y1 = (y_index as f64 * y_ratio) as u32;
                        let x2 =
                            ((x1 + 1).min((self.image.get_width() - 1).try_into().unwrap())) as u32;
                        let y2 = ((y1 + 1).min((self.image.get_height() - 1).try_into().unwrap()))
                            as u32;
                        let x_diff = (x_index as f64 * x_ratio) as f64
                            - ((x_index as f64 * x_ratio) as u32) as f64;
                        let y_diff = (y_index as f64 * y_ratio) as f64
                            - ((y_index as f64 * y_ratio) as u32) as f64;

                        let top_left = self.image.get_pixel_at(x1, y1).unwrap();
                        let top_right = self.image.get_pixel_at(x2, y1).unwrap();
                        let bottom_left = self.image.get_pixel_at(x1, y2).unwrap();
                        let bottom_right = self.image.get_pixel_at(x2, y2).unwrap();

                        let top = top_left.clone() + (top_right - top_left) * x_diff;
                        let bottom = bottom_left.clone() + (bottom_right - bottom_left) * x_diff;
                        let pix = (top).clone() + (bottom - top) * y_diff;
                        return pix;
                    })
                    .collect::<Vec<Pixels>>()
            })
            .collect::<Vec<Pixels>>();

        for pix in pixel_list.iter() {
            new_image.add_pixel(pix.clone());
        }

        new_image
    }
}
