use super::pixel::Pixels;

#[derive(Debug, Clone)]
pub struct Images<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    width: u32,
    height: u32,
    channels: u8,
    image_data: Vec<Pixels<T>>,
}

impl<T> Images<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_channels(&self) -> u8 {
        self.channels
    }

    pub fn get_image(&self) -> Vec<Pixels<T>> {
        // If i use self instead of &self, we take ownership of self
        self.image_data.clone()
    }

    pub fn new(width: u32, height: u32, channels: u8, image_data: Vec<Pixels<T>>) -> Self {
        Self {
            width,
            height,
            channels,
            image_data,
        }
    }

    pub fn get_pixel_at(&self, x: u32, y: u32) -> Result<Pixels<T>, Box<dyn std::error::Error>> {
        if x >= self.width || y >= self.height {
            return Err(format!("Coordinates out of bounds for x:{:?} and y {:?}", x, y).into());
        }

        let location = y * self.width + x;
        match self.image_data.get(location as usize) {
            Some(pixel) => Ok(pixel.clone()),
            None => Err(("Pixel not found").into()),
        }
    }

    pub fn set_pixel_at(
        &mut self,
        x: u32,
        y: u32,
        pixel: Pixels<T>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let location = x * self.width + y;
        if self.image_data.len() < (location - 1).try_into().unwrap() {
            return Err("Location more than the image length".into());
        }
        self.image_data.insert(location as usize, pixel);

        Ok(())
    }

    pub fn add_pixel(&mut self, pix: Pixels<T>) {
        self.image_data.push(pix);
    }
}

impl<T> PartialEq for Images<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.channels == other.channels
            && self.image_data == other.image_data
        // && self.image_data.len() == other.image_data.len()
        // && self.image_data.iter().zip(other.image_data.iter()).all(|(a,b)| *a == *b)
    }
}
