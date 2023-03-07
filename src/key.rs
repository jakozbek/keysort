use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use crate::{decision::Characteristic, plant::Plant};

use anyhow::Result;

#[derive(Debug)]
struct OptionNode {
    pub left_node: Option<u32>,
    pub right_node: Option<u32>,
    possibilities: Vec<Plant>,
    characteristic: Option<String>,
}

#[derive(Debug)]
struct PlantNode {
    plant: Plant,
}

impl Display for PlantNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.plant)
    }
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
            left_node: None,
            right_node: None,
            possibilities: plants,
            // arrangement is initial characteristic
            characteristic: characteristic_order.pop_front(),
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

            let mut left_child_node: Option<u32> = None;
            let mut right_child_node: Option<u32> = None;

            {
                let current_node = nodes.get(&current_index).unwrap();

                let next_characteristic = characteristic_order.pop_front();

                match current_node {
                    Node::Option(option_node) => {
                        let characteristic = match &option_node.characteristic {
                            Some(characteristic) => characteristic,
                            None => {
                                // exit loop
                                continue;
                            }
                        };

                        let mut true_group = Vec::new();
                        let mut false_group = Vec::new();

                        for plant in &option_node.possibilities {
                            let plant_characteristics = &plant.characteristics;

                            // TODO: handle unwrap
                            let decision = plant_characteristics.get(characteristic).unwrap();

                            if decision.decide() {
                                true_group.push(plant.clone());
                            } else {
                                false_group.push(plant.clone());
                            }
                        }

                        for boolean in [true, false] {
                            // increment the index for the next node
                            index = index + 1;

                            let mut group = if boolean {
                                true_group.clone()
                            } else {
                                false_group.clone()
                            };

                            let node = if group.len() == 1 {
                                let plant = group
                                    .pop()
                                    .expect("Plant will always exist because we check the lenght");

                                let plant_node = PlantNode { plant };

                                Node::Plant(plant_node)
                            } else {
                                // left and right node will be added when children are created
                                let option_node = OptionNode {
                                    left_node: None,
                                    right_node: None,
                                    possibilities: group,
                                    characteristic: next_characteristic.clone(),
                                };

                                indexes_to_check.push_back(index);

                                Node::Option(option_node)
                            };

                            nodes.insert(index, node);

                            if boolean {
                                left_child_node = Some(index);
                            } else {
                                right_child_node = Some(index);
                            }
                        }
                    }
                    Node::Plant(_) => {}
                }
            }

            let mutable_node_current = nodes.get_mut(&current_index).unwrap();

            match mutable_node_current {
                Node::Option(option_node) => {
                    option_node.left_node = left_child_node;
                    option_node.right_node = right_child_node;
                }
                Node::Plant(_) => {}
            }
        }
    }
}

// left is true, right is false
fn pre_order_traversal(
    node: &Node,
    nodes: &KeyNodes,
    f: &mut std::fmt::Formatter<'_>,
    index: &mut u32,
    depth: u32,
) {
    for _ in 0..depth {
        write!(f, " ").unwrap();
    }

    match node {
        Node::Option(option_node) => {
            *index = *index + 1;
            let node_index = *index;

            let node_characteristic = option_node.characteristic.as_ref().unwrap();

            write!(
                f,
                "{}: {} is {}\n",
                node_index,
                node_characteristic,
                Characteristic::show_option(node_characteristic, true)
            )
            .unwrap();

            if let Some(left_node_idx) = option_node.left_node {
                let left_node = nodes.get(&left_node_idx).unwrap();
                pre_order_traversal(&left_node, nodes, f, index, node_index + 1);
            }

            for _ in 0..depth {
                write!(f, " ").unwrap();
            }

            write!(
                f,
                "{}: {} is {}\n",
                node_index,
                node_characteristic,
                Characteristic::show_option(node_characteristic, false)
            )
            .unwrap();

            if let Some(right_node_idx) = option_node.right_node {
                let right_node = nodes.get(&right_node_idx).unwrap();
                pre_order_traversal(&right_node, nodes, f, index, node_index + 1);
            }
        }
        Node::Plant(plant_node) => {
            write!(f, "{}\n", plant_node.plant).unwrap();
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut global_index = 0;

        pre_order_traversal(
            &self.nodes.get(&0).unwrap(),
            &self.nodes,
            f,
            &mut global_index,
            0,
        );

        Ok(())
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
