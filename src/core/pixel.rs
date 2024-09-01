use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Pixels<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    red: T,
    green: T,
    blue: T,
    alpha: T,
}

impl<T> Pixels<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    pub fn get_red(&self) -> T {
        self.red
    }

    pub fn get_green(&self) -> T {
        self.green
    }

    pub fn get_blue(&self) -> T {
        self.blue
    }

    pub fn get_alpha(&self) -> T {
        self.alpha
    }

    pub fn new(red: T, green: T, blue: T, alpha: T) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl<T> Default for Pixels<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    fn default() -> Self {
        Self {
            red: 0.into(),
            green: 0.into(),
            blue: 0.into(),
            alpha: 255.into(),
        }
    }
}

impl<T> Add for Pixels<T>
where
    T: Copy + Clone + From<u8> + Into<f64> + std::cmp::PartialEq,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: ((self.red.into() + rhs.red.into()) as u8)
                .clamp(0, 255)
                .into(),
            green: ((self.green.into() + rhs.green.into()) as u8)
                .clamp(0, 255)
                .into(),
            blue: ((self.blue.into() + rhs.blue.into()) as u8)
                .clamp(0, 255)
                .into(),
            alpha: ((self.alpha.into() + rhs.alpha.into()) as u8)
                .clamp(0, 255)
                .into(),
        }
    }
}

impl<T> Sub for Pixels<T>
where
    T: Copy + Clone + From<u8> + Into<f64> + std::cmp::PartialEq,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: ((self.red.into() - rhs.red.into()) as u8)
                .clamp(0, 255)
                .into(),
            green: ((self.green.into() - rhs.green.into()) as u8)
                .clamp(0, 255)
                .into(),
            blue: ((self.blue.into() - rhs.blue.into()) as u8)
                .clamp(0, 255)
                .into(),
            alpha: ((self.alpha.into() - rhs.alpha.into()) as u8)
                .clamp(0, 255)
                .into(),
        }
    }
}

impl<T> Mul<f64> for Pixels<T>
where
    T: Copy + Clone + From<u8> + Into<f64> + std::cmp::PartialEq,
{
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: ((self.red.into() * rhs) as u8).clamp(0, 255).into(),
            green: ((self.green.into() * rhs) as u8)
                .clamp(0, 255)
                .into(),
            blue: ((self.blue.into() * rhs) as u8).clamp(0, 255).into(),
            alpha: ((self.alpha.into() * rhs) as u8)
                .clamp(0, 255)
                .into(),
        }
    }
}

impl<T> Mul<u32> for Pixels<T>
where
    T: Copy + Clone + From<u8> + Into<u32> + std::cmp::PartialEq,
{
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            red: ((self.red.into() * rhs) as u8).clamp(0, 255).into(),
            green: ((self.green.into() * rhs) as u8)
                .clamp(0, 255)
                .into(),
            blue: ((self.blue.into() * rhs) as u8).clamp(0, 255).into(),
            alpha: ((self.alpha.into() * rhs) as u8)
                .clamp(0, 255)
                .into(),
        }
    }
}

impl<T> Div<u8> for Pixels<T>
where
    T: Copy + Clone + From<u8> + Into<u32> + std::cmp::PartialEq,
{
    type Output = Self;
    fn div(self, rhs: u8) -> Self::Output {
        Self {
            red: ((self.red.into() / rhs as u32) as u8)
                .clamp(0, 255)
                .into(),
            green: ((self.green.into() / rhs as u32) as u8)
                .clamp(0, 255)
                .into(),
            blue: ((self.blue.into() / rhs as u32) as u8)
                .clamp(0, 255)
                .into(),
            alpha: ((self.alpha.into() / rhs as u32) as u8)
                .clamp(0, 255)
                .into(),
        }
    }
}

impl<T> PartialEq for Pixels<T>
where
    T: Copy + Clone + From<u8> + Into<f64> + std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red
            && self.green == other.green
            && self.blue == other.blue
            && self.alpha == other.alpha
    }
}
