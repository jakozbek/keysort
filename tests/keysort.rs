use std::fs;

use keysort::{characteristic::Characteristic, key::Key, plant::Plant};

#[test]
fn main() {
    let characteristics_file_string =
        fs::read_to_string("test_data/test_characteristics.json").expect("Unable to read file");

    let characteristics: Vec<Characteristic> = serde_json::from_str(&characteristics_file_string)
        .expect("Unable to parse characteristics file");

    let plants_file_string =
        fs::read_to_string("test_data/test_plants.json").expect("Unable to read file");

    let plants: Vec<Plant> =
        serde_json::from_str(&plants_file_string).expect("Unable to parse characteristics file");

    let mut key = Key::new();
    key.build(&plants, &characteristics).unwrap();

    println!("{:?}", key);
    println!("{}", key);
}
