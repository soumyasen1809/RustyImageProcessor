use std::ops::{Add, Div, Mul, Sub};

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

impl Default for Pixels {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }
}

impl Add for Pixels {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: (self.red + rhs.red).clamp(0, 255),
            green: (self.green + rhs.green).clamp(0, 255),
            blue: (self.blue + rhs.blue).clamp(0, 255),
            alpha: (self.alpha + rhs.alpha).clamp(0, 255),
        }
    }
}

impl Sub for Pixels {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: (self.red - rhs.red).clamp(0, 255),
            green: (self.green - rhs.green).clamp(0, 255),
            blue: (self.blue - rhs.blue).clamp(0, 255),
            alpha: (self.alpha - rhs.alpha).clamp(0, 255),
        }
    }
}

impl Mul<f64> for Pixels {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: ((self.red as f64 * rhs) as u8).clamp(0, 255),
            green: ((self.green as f64 * rhs) as u8).clamp(0, 255),
            blue: ((self.blue as f64 * rhs) as u8).clamp(0, 255),
            alpha: ((self.alpha as f64 * rhs) as u8).clamp(0, 255),
        }
    }
}

impl Mul<u32> for Pixels {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            red: ((self.red as u32 * (rhs as u32)) as u8).clamp(0, 255),
            green: ((self.green as u32 * (rhs as u32)) as u8).clamp(0, 255),
            blue: ((self.blue as u32 * (rhs as u32)) as u8).clamp(0, 255),
            alpha: ((self.alpha as u32 * (rhs as u32)) as u8).clamp(0, 255),
        }
    }
}

impl Div<u8> for Pixels {
    type Output = Self;
    fn div(self, rhs: u8) -> Self::Output {
        Self {
            red: ((self.red as u32 / rhs as u32) as u8).clamp(0, 255),
            green: ((self.green as u32 / rhs as u32) as u8).clamp(0, 255),
            blue: ((self.blue as u32 / rhs as u32) as u8).clamp(0, 255),
            alpha: ((self.alpha as u32 / rhs as u32) as u8).clamp(0, 255),
        }
    }
}

impl PartialEq for Pixels {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red
            && self.green == other.green
            && self.blue == other.blue
            && self.alpha == other.alpha
    }
}
