use super::pixel::Pixels;

#[derive(Debug, Clone)]
pub struct Images {
    width: u32,
    height: u32,
    channels: u8,
    image_data: Vec<Pixels>,
}

impl Images {
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_channels(&self) -> u8 {
        self.channels
    }

    pub fn get_image(&self) -> Vec<Pixels> {
        // If i use self instead of &self, we take ownership of self
        self.image_data.clone()
    }

    pub fn new(width: u32, height: u32, channels: u8, image_data: Vec<Pixels>) -> Self {
        Self {
            width,
            height,
            channels,
            image_data,
        }
    }

    pub fn get_pixel_at(&self, x: u32, y: u32) -> Result<Pixels, Box<dyn std::error::Error>> {
        let location = y * self.width + x;
        let pixel = self.image_data.get(location as usize);

        Ok(pixel.unwrap().clone())
    }

    pub fn set_pixel_at(
        &mut self,
        x: u32,
        y: u32,
        pixel: Pixels,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let location = x * self.width + y;
        if self.image_data.len() < (location - 1).try_into().unwrap() {
            return Err("location more than the image length".into());
        }
        self.image_data.insert(location as usize, pixel);

        Ok(())
    }

    pub fn add_pixel(&mut self, pix: Pixels) {
        self.image_data.push(pix);
    }
}
