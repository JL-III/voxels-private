use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum Element {
    Air,
    Dirt,
    Grass,
    Stone,
}

impl Element {
    pub fn from_string(s: &str) -> Option<Element> {
        match s {
            "air" => Some(Element::Air),
            "dirt" => Some(Element::Dirt),
            "grass" => Some(Element::Grass),
            "stone" => Some(Element::Stone),
            _ => None, // Handle the case where the string doesn't match any variant
        }
    }
}
