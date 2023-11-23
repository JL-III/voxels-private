#[derive(Copy, Debug, PartialEq)]
pub enum Element {
    Air,
    Dirt,
    Grass,
    Stone,
}

impl Clone for Element {
    fn clone(&self) -> Self {
        *self
    }
}
