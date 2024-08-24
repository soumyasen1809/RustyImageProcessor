#[derive(Debug, Clone)]
pub struct Pixels {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Pixels {
    pub fn get_red(&self) -> u8 {
        self.red
    }

    pub fn get_green(&self) -> u8 {
        self.green
    }

    pub fn get_blue(&self) -> u8 {
        self.blue
    }

    pub fn get_alpha(&self) -> u8 {
        self.alpha
    }

    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}
