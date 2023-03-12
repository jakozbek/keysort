use std::fs;

use keysort::{characteristic::Characteristic, plant::Plant, key::Key};

#[test]
fn main() {
    let characteristics_file_string =
        fs::read_to_string("test_data/test_characteristics.json").expect("Unable to read file");

    let characteristics: Vec<Characteristic> = serde_json::from_str(&characteristics_file_string)
        .expect("Unable to parse characteristics file");

    let plants_file_string =
        fs::read_to_string("test_data/test_plants.json").expect("Unable to read file");

    let plants: Vec<Plant> = serde_json::from_str(&plants_file_string)
        .expect("Unable to parse characteristics file");

    let key = Key::build(&plants, &characteristics).unwrap();

    println!("{}", key)
}
