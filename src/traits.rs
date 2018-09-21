pub trait Position{
    fn position(&self) -> f32;
}
impl<'a, T> Position for &'a T where T: Position{
    fn position(&self) -> f32{
        (*self).position()
    }
}
impl<'a, T> Position for &'a mut T where T: Position{
    fn position(&self) -> f32{
        (**self).position()
    }
}