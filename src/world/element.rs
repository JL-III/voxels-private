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

impl Element {
    pub fn from_str(s: &str) -> Option<Element> {
        match s {
            "air" => Some(Element::Air),
            "dirt" => Some(Element::Dirt),
            "grass" => Some(Element::Grass),
            "stone" => Some(Element::Stone),
            _ => None, // Handle the case where the string doesn't match any variant
        }
    }
}
