use crate::{
    core::{image::Images, pixel::Pixels},
    transformations::rotate::Transformation,
};

pub struct BoxBlur {
    image: Images,
}

impl BoxBlur {
    pub fn new(image: &Images) -> Self {
        Self {
            image: image.clone(),
        }
    }
}

// AI: Algorithm from Gemini
impl Transformation for BoxBlur {
    fn apply(&self) -> Images {
        let kernel: Vec<u8> = vec![1, 2, 1, 2, 4, 2, 1, 2, 1]; // Defined kernel: // Gaussian blur kernel for better smoothing
        let kernel_size: u32 = 3;
        // let kernel_sum: u32 = kernel.iter().map(|x| *x as u32).sum();
        let kernel_max: u32 = *kernel.iter().max().unwrap() as u32;
        let half_kernel_size = kernel_size / 2;

        let output_width = self.image.get_width() - 2 * half_kernel_size;
        let output_height = self.image.get_height() - 2 * half_kernel_size;

        let mut output_image = Images::new(
            output_width,
            output_height,
            self.image.get_channels(),
            Vec::new(),
        );

        for y in half_kernel_size..self.image.get_height() - half_kernel_size {
            for x in half_kernel_size..self.image.get_width() - half_kernel_size {
                let mut sum_r = 0;
                let mut sum_g = 0;
                let mut sum_b = 0;

                for dy in 0..=half_kernel_size {
                    for dx in 0..=half_kernel_size {
                        let pixel = self
                            .image
                            .get_pixel_at(
                                x as u32 + dx - half_kernel_size,
                                y as u32 + dy - half_kernel_size,
                            )
                            .unwrap();
                        let kernel_val = kernel[(dy * kernel_size + dx) as usize];
                        sum_r += (pixel.get_red() as u32) * (kernel_val as u32);
                        sum_g += (pixel.get_green() as u32) * (kernel_val as u32);
                        sum_b += (pixel.get_blue() as u32) * (kernel_val as u32);
                    }
                }

                let new_pixel = Pixels::new(
                    (sum_r / kernel_max) as u8,
                    (sum_g / kernel_max) as u8,
                    (sum_b / kernel_max) as u8,
                    255,
                );
                // In the case of box blurring, the goal is to smooth the image, and transparency is not typically involved.
                // Therefore, setting the alpha to 255 is appropriate.
                // Changing it to some other value, will create grey white boxes

                output_image.add_pixel(new_pixel);
            }
        }

        output_image
    }
}
