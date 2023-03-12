use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Characteristic {
    pub name: String,
    pub true_option: String,
    pub false_option: String,
}

impl Characteristic {
    pub fn display_for_key(
        &self,
        option: bool,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if option {
            write!(f, "{}", self.true_option)
        } else {
            write!(f, "{}", self.false_option)
        }
    }

    pub fn parse(value: &str) -> Result<Characteristic> {
        let c: Characteristic = serde_json::from_str(value)?;
        Ok(c)
    }
}

pub trait KeyDecision {
    fn get_decision(&self, value: &str) -> Result<bool>;
}

impl KeyDecision for Characteristic {
    fn get_decision(&self, value: &str) -> Result<bool> {
        if value == self.true_option {
            Ok(true)
        } else if value == self.false_option {
            Ok(false)
        } else {
            Err(anyhow!("Invalid value"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"
        {
            "name": "test",
            "true_option": "test_1",
            "false_option": "test_2"
        }
        "#;

        let c = Characteristic::parse(json).unwrap();

        assert_eq!(c.name, "test");
        assert_eq!(c.true_option, "test_1");
        assert_eq!(c.false_option, "test_2");
    }
}
