use std::collections::{HashMap, VecDeque};

use crate::plant::Plant;

use anyhow::Result;

#[derive(Debug)]
struct OptionNode {
    prev_node: Option<u32>,
    possibilities: Vec<Plant>,
    characteristic: String,
}

#[derive(Debug)]
struct PlantNode {
    plant: Plant,
    prev_node: u32,
}

#[derive(Debug)]
enum Node {
    Option(OptionNode),
    Plant(PlantNode),
}

type KeyNodes = HashMap<u32, Node>;

#[derive(Debug)]
pub struct Key {
    nodes: KeyNodes,
}

impl Key {
    pub fn build(plants: Vec<Plant>, characteristics: Vec<String>) -> Result<Key> {
        let mut characteristic_order: VecDeque<String> = VecDeque::from_iter(characteristics);

        let mut index = 0;

        let mut indexes_to_check: VecDeque<u32> = VecDeque::new();

        let initial_option = OptionNode {
            prev_node: None,
            possibilities: plants,
            // arrangement is initial characteristic
            characteristic: String::from(characteristic_order.pop_front().unwrap()),
        };

        let mut nodes = KeyNodes::new();

        nodes.insert(index, Node::Option(initial_option));

        // start by checking index 0
        indexes_to_check.push_front(0);

        loop {
            if indexes_to_check.is_empty() {
                break Ok(Key { nodes });
            }

            let current_index = indexes_to_check.pop_front().unwrap();

            let current_node = nodes.get(&current_index).unwrap();

            let next_characteristic = match characteristic_order.pop_front() {
                Some(characteristic) => characteristic,
                None => {
                    // exit loop
                    break Ok(Key { nodes });
                }
            };

            match current_node {
                Node::Option(option) => {
                    let characteristic = &option.characteristic;

                    let mut true_group = Vec::new();
                    let mut false_group = Vec::new();

                    for plant in &option.possibilities {
                        let plant_characteristics = &plant.characteristics;

                        // TODO: handle unwrap
                        let decision = plant_characteristics.get(characteristic).unwrap();

                        if decision.decide() {
                            true_group.push(plant.clone());
                        } else {
                            false_group.push(plant.clone());
                        }
                    }

                    // increment the index for the next node
                    index = index + 1;

                    for boolean in [true, false] {
                        let mut group = if boolean {
                            true_group.clone()
                        } else {
                            false_group.clone()
                        };

                        if group.len() == 1 {
                            let plant = group.pop()
                                .expect("Plant will always exist because we check the lenght");

                            let plant_node = PlantNode {
                                plant,
                                prev_node: current_index,
                            };

                            nodes.insert(index, Node::Plant(plant_node));
                        } else {
                            let true_option = OptionNode {
                                prev_node: Some(current_index),
                                possibilities: group,
                                characteristic: next_characteristic.clone(),
                            };

                            indexes_to_check.push_back(index);

                            nodes.insert(index, Node::Option(true_option));
                        }
                    }
                }
                Node::Plant(_) => {}
            }
        }
    }
}

// TODO
// mod test {
//     use super::*;


    // TODO: test for plants that won't be sorted completely because there are no more characteristics
    // #[test]
    // mod build_happy_path {}

    // TODO: test for more characteristics than needed
// }
