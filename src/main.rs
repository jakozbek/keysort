use keysort::key::Key;
use keysort::test_data::get_test_data;

fn main() {
    let test_plants = get_test_data();

    let characteristics_order = vec!["arrangement".to_string(), "leaf_type".to_string()];

    let key = Key::build(test_plants, characteristics_order).unwrap();

    println!("{}", key);
}
