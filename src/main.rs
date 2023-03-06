use keysort::key::Key;
use keysort::test_data::get_test_data;

fn main() {
    let test_plants = get_test_data();

    let key = Key::build(test_plants).unwrap();

    println!("{:?}", key);
}
