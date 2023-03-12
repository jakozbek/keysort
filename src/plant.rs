use std::collections::HashMap;
use std::fmt::{Debug, Display};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Plant {
    genus: String,
    species: String,
    #[serde(with = "chars")]
    pub characteristics: HashMap<String, String>
}

impl Display for Plant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.genus, self.species)
    }
}


mod chars {
    use super::*;
    use std::collections::HashMap;
    use serde::de::{Deserialize, Deserializer};

    // Intermediate struct to deserialize into
    #[derive(Deserialize)]
    struct CharacteristicsData {
        name: String,
        value: String
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
        where D: Deserializer<'de>
    {
        let mut map = HashMap::new();
        for item in Vec::<CharacteristicsData>::deserialize(deserializer)? {
            map.insert(item.name, item.value);
        }
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_plant() {
        let json = r#"
        {
            "genus": "Aloe",
            "species": "vera",
            "characteristics": [
                {
                    "name": "height",
                    "value": "1m"
                },
                {
                    "name": "width",
                    "value": "2m"
                }
            ]
        }
        "#;

        let plant: Plant = serde_json::from_str(json).unwrap();
        assert_eq!(plant.genus, "Aloe");
        assert_eq!(plant.species, "vera");
        assert_eq!(plant.characteristics.len(), 2);
        assert_eq!(plant.characteristics.get("height").unwrap(), "1m");
        assert_eq!(plant.characteristics.get("width").unwrap(), "2m");
    }

    #[test]
    fn test_deserialize_plants() {
        let json = r#"
        [
            {
                "genus": "Aloe",
                "species": "vera",
                "characteristics": [
                    {
                        "name": "height",
                        "value": "1m"
                    },
                    {
                        "name": "width",
                        "value": "1m"
                    }
                ]
            },
            {
                "genus": "Aloe",
                "species": "vera",
                "characteristics": [
                    {
                        "name": "height",
                        "value": "1m"
                    },
                    {
                        "name": "width",
                        "value": "1m"
                    }
                ]
            }
        ]
        "#;

        let plants: Vec<Plant> = serde_json::from_str(json).unwrap();
        assert_eq!(plants.len(), 2);
    }
}
