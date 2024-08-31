use super::image::Images;

pub trait Operation<T> {
    fn apply(&self) -> Images<T>;
}
