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
        if x >= self.width || y >= self.height {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Coordinates out of bounds for x:{:?} and y {:?}", x, y),
            )));
        }

        let location = y * self.width + x;
        match self.image_data.get(location as usize) {
            Some(pixel) => Ok(pixel.clone()),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Pixel not found",
            ))),
        }
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

impl PartialEq for Images {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.channels == other.channels
            && self.image_data == other.image_data
    }
}
