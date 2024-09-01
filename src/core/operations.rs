use super::image::Images;

pub trait Operation<T>
where
    T: Copy + Clone + From<u8> + std::cmp::PartialEq,
{
    fn apply(&self) -> Images<T>;
}
