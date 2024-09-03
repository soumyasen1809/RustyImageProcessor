use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::core::{image::Images, operations::Operation, pixel::Pixels};

pub struct ResizeNearestNeighbour {
    new_width: u32,
    new_height: u32,
}

impl ResizeNearestNeighbour {
    pub fn new(new_width: u32, new_height: u32) -> Self {
        Self {
            new_width,
            new_height,
        }
    }
}

impl<T> Operation<T> for ResizeNearestNeighbour
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let mut new_image = Images::new(
            self.new_width,
            self.new_height,
            old_image.get_channels(),
            Vec::new(),
        );

        let x_ratio = old_image.get_width() as f64 / self.new_width as f64;
        let y_ratio = old_image.get_height() as f64 / self.new_height as f64;

        let pixel_list = (0..self.new_height)
            .into_par_iter()
            .flat_map(|y_index| {
                (0..self.new_width)
                    .into_par_iter()
                    .map(move |x_index| {
                        old_image
                            .get_pixel_at(
                                (x_index as f64 * x_ratio) as u32,
                                (y_index as f64 * y_ratio) as u32,
                            )
                            .unwrap()
                    })
                    .collect::<Vec<Pixels<T>>>()
            })
            .collect::<Vec<Pixels<T>>>();

        for pix in pixel_list.iter() {
            new_image.add_pixel(pix.clone());
        }

        new_image
    }
}

pub struct ResizeBilinearInterpolation {
    new_width: u32,
    new_height: u32,
}

impl ResizeBilinearInterpolation {
    pub fn new(new_width: u32, new_height: u32) -> Self {
        Self {
            new_width,
            new_height,
        }
    }
}

impl<T> Operation<T> for ResizeBilinearInterpolation
where
    T: Copy + Clone + From<u8> + Into<f64> + std::cmp::PartialEq + Send + Sync,
{
    fn apply(&self, old_image: &Images<T>) -> Images<T> {
        let x_ratio = old_image.get_width() as f64 / self.new_width as f64;
        let y_ratio = old_image.get_height() as f64 / self.new_height as f64;

        let new_pixel = (0..self.new_height)
            .into_par_iter()
            .flat_map(|y_index| {
                (0..self.new_width)
                    .into_par_iter()
                    .map(|x_index| {
                        let x1 = (x_index as f64 * x_ratio) as u32;
                        let y1 = (y_index as f64 * y_ratio) as u32;
                        let x2 = (x1 + 1).min(old_image.get_width() - 1);
                        let y2 = (y1 + 1).min(old_image.get_height() - 1);
                        let x_diff =
                            (x_index as f64 * x_ratio) - ((x_index as f64 * x_ratio) as u32) as f64;
                        let y_diff =
                            (y_index as f64 * y_ratio) - ((y_index as f64 * y_ratio) as u32) as f64;

                        let top_left: Pixels<T> = old_image.get_pixel_at(x1, y1).unwrap();
                        let top_right: Pixels<T> = old_image.get_pixel_at(x2, y1).unwrap();
                        let bottom_left: Pixels<T> = old_image.get_pixel_at(x1, y2).unwrap();
                        let bottom_right: Pixels<T> = old_image.get_pixel_at(x2, y2).unwrap();

                        let top: Pixels<T> = top_left.clone() + (top_right - top_left) * x_diff;
                        let bottom: Pixels<T> =
                            bottom_left.clone() + (bottom_right - bottom_left) * x_diff;

                        (top).clone() + (bottom - top) * y_diff
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
