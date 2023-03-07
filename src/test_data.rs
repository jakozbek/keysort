use crate::{
    decision::{Arrangement, Decision, LeafType},
    plant::Plant,
};
use std::collections::HashMap;

// create function to return test_data
pub fn get_test_data() -> Vec<Plant> {
    let characteristics = {
        let mut characteristics = HashMap::new();

        characteristics.insert(
            String::from("arrangement"),
            Box::new(Arrangement::Opposite) as Box<dyn Decision>,
        );
        characteristics.insert(
            String::from("leaf_type"),
            Box::new(LeafType::Simple) as Box<dyn Decision>,
        );
        characteristics
    };

    let maple = Plant::new(
        String::from("Acer"),
        String::from("rubrum"),
        characteristics,
    );

    let characteristics = {
        let mut characteristics = HashMap::new();

        characteristics.insert(
            String::from("arrangement"),
            Box::new(Arrangement::Opposite) as Box<dyn Decision>,
        );
        characteristics.insert(
            String::from("leaf_type"),
            Box::new(LeafType::Compound) as Box<dyn Decision>,
        );
        characteristics
    };

    let ash = Plant::new(
        String::from("Fraxinus"),
        String::from("americana"),
        characteristics,
    );

    let characteristics = {
        let mut characteristics = HashMap::new();

        characteristics.insert(
            String::from("arrangement"),
            Box::new(Arrangement::Alternate) as Box<dyn Decision>,
        );
        characteristics.insert(
            String::from("leaf_type"),
            Box::new(LeafType::Simple) as Box<dyn Decision>,
        );
        characteristics
    };

    let cherry = Plant::new(
        String::from("Prunus"),
        String::from("serotina"),
        characteristics,
    );

    let characteristics = {
        let mut characteristics = HashMap::new();

        characteristics.insert(
            String::from("arrangement"),
            Box::new(Arrangement::Alternate) as Box<dyn Decision>,
        );
        characteristics.insert(
            String::from("leaf_type"),
            Box::new(LeafType::Compound) as Box<dyn Decision>,
        );
        characteristics
    };

    let hickory = Plant::new(
        String::from("Carya"),
        String::from("ovata"),
        characteristics,
    );

    vec![cherry, hickory, ash, maple]
}
