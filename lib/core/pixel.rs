use std::ops::{Add, Mul, Sub};

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

impl Add for Pixels {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            alpha: self.alpha + rhs.alpha,
        }
    }
}

impl Sub for Pixels {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
            alpha: self.alpha - rhs.alpha,
        }
    }
}

impl Mul<f64> for Pixels {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: (self.red as f64 * rhs) as u8,
            green: (self.green as f64 * rhs) as u8,
            blue: (self.blue as f64 * rhs) as u8,
            alpha: (self.alpha as f64 * rhs) as u8,
        }
    }
}
