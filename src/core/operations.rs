use super::image::Images;

pub trait Operation {
    fn apply(&self) -> Images;
}
