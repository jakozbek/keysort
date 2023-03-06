use std::{collections::HashMap, fmt::Display};

use crate::decision::Decision;

#[derive(Clone, Debug)]
pub struct Plant {
    genus: String,
    species: String,
    pub characteristics: Characteristics,
}

impl Plant {
    pub fn new(genus: String, species: String, characteristics: Characteristics) -> Self {
        Self {
            genus,
            species,
            characteristics,
        }
    }
}

impl Display for Plant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.genus, self.species)
    }
}

type Characteristics = HashMap<String, Box<dyn Decision>>;
