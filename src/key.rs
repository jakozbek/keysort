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
    pub fn build(plants: Vec<Plant>) -> Result<Key> {
        // create a new VecDeque of characteristics to check
        let char_possibilities = ["arrangement", "leaf_type"];

        let mut characteristic_order: VecDeque<&str> = VecDeque::from_iter(char_possibilities);

        let mut nodes = KeyNodes::new();

        let mut index = 0;

        // TODO: make VecDeque
        let mut indexes_to_check: VecDeque<u32> = VecDeque::new();

        let initial_option = OptionNode {
            prev_node: None,
            possibilities: plants,
            // arrangement is initial characteristic
            characteristic: String::from(characteristic_order.pop_front().unwrap()),
        };

        nodes.insert(index, Node::Option(initial_option));

        // start by checking index 0
        indexes_to_check.push_front(0);

        loop {
            if indexes_to_check.is_empty() {
                // exit loop
                break Ok(Key { nodes });
            }

            // TOOD: pop
            let current_index = indexes_to_check[0];

            let current_node = nodes.get(&current_index).unwrap();

            let next_characteristic = characteristic_order.pop_front();

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

                    // TODO: should we increment index for a plant node?
                    index = index + 1;

                    if true_group.len() == 1 {
                        let plant = true_group
                            .pop()
                            .expect("Plant will always exist because we check the lenght");

                        let plant_node = PlantNode {
                            plant,
                            prev_node: current_index,
                        };

                        nodes.insert(index, Node::Plant(plant_node));
                    } else {
                        let true_option = OptionNode {
                            prev_node: Some(current_index),
                            possibilities: true_group,
                            characteristic: next_characteristic.unwrap().to_string(),
                        };

                        indexes_to_check.push_back(index);

                        nodes.insert(index, Node::Option(true_option));
                    }

                    index = index + 1;

                    if false_group.len() == 1 {
                        let plant = false_group
                            .pop()
                            .expect("Plant will always exist because we check the lenght");

                        let plant_node = PlantNode {
                            plant,
                            prev_node: current_index,
                        };

                        nodes.insert(index, Node::Plant(plant_node));
                    } else {
                        let false_option = OptionNode {
                            prev_node: Some(current_index),
                            possibilities: false_group,
                            characteristic: next_characteristic.unwrap().to_string(),
                        };

                        indexes_to_check.push_back(index);

                        nodes.insert(index, Node::Option(false_option));
                    }
                }
                Node::Plant(_) => {}
            }

            indexes_to_check.pop_front();
        }
    }
}
