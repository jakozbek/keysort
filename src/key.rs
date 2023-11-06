use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
};

use crate::characteristic::{Characteristic, KeyDecision};
use crate::plant::Plant;

use anyhow::Result;

#[derive(Clone)]
struct OptionNode {
    // TODO: is there a way to not have to include this
    pub index: u32,
    pub left_node: Option<u32>,
    pub right_node: Option<u32>,
    possibilities: Vec<Plant>,
    // TODO: not sure why I made option
    pub characteristic: Option<Characteristic>,
}

impl OptionNode {
    pub fn insert_child_index(&mut self, node_index: u32) {
        if self.left_node.is_none() {
            self.left_node = Some(node_index);
        } else if self.right_node.is_none() {
            self.right_node = Some(node_index);
        } else {
            let msg = format!("Option node is full at index {}", self.index);
            panic!("{}", msg)
        }
    }
}

impl Debug for OptionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let characteristic = match &self.characteristic {
            Some(characteristic) => characteristic.name.clone(),
            None => "None".to_string(),
        };
        write!(f, "OptionNode {{ index: {}, left_node: {:?}, right_node: {:?}, possibilities: {:?}, characteristic: {:?} }}\n", self.index, self.left_node, self.right_node, self.possibilities.len(), characteristic)
    }
}

#[derive(Clone)]
struct PlantNode {
    index: u32,
    plant: Plant,
}

impl Debug for PlantNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PlantNode {{ plant: {:?} }}", self.plant)
    }
}

impl Display for PlantNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.plant)
    }
}

#[derive(Debug, Clone)]
enum Node {
    Option(OptionNode),
    Plant(PlantNode),
}

impl Node {
    pub fn get_index(&self) -> u32 {
        match self {
            Node::Option(option_node) => option_node.index,
            Node::Plant(plant_node) => plant_node.index,
        }
    }
}

type KeyNodes = HashMap<u32, Node>;

#[derive(Debug, Default)]
pub struct Key {
    current_node_index: u32,
    nodes: KeyNodes,
}

fn handle_option_node(
    option_node: &mut OptionNode,
    next_characteristic: Option<&Characteristic>,
    mut current_node_index: u32,
) -> Result<Vec<Node>> {
    let mut nodes = vec![];

    let characteristic = &option_node.characteristic.clone().unwrap_or_else(|| {
        panic!(
            "Characteristic should exist at node {:?} with possibilities: {:?}",
            option_node, option_node.possibilities
        )
    });

    let mut true_group = Vec::new();
    let mut false_group = Vec::new();

    let mut plants_without_characteristic = 0;

    for plant in &option_node.possibilities {
        let plant_characteristics = &plant.characteristics;

        let char_to_check = match plant_characteristics.get(&characteristic.name) {
            Some(characteristic) => characteristic,
            None => {
                plants_without_characteristic = plants_without_characteristic + 1;
                continue;
            }
        };

        let decision = characteristic.get_decision(char_to_check)?;

        if decision {
            true_group.push(plant.clone());
        } else {
            false_group.push(plant.clone());
        }
    }

    if plants_without_characteristic > 0 {
        if plants_without_characteristic == option_node.possibilities.len() {
            // if we cannot separate based on existing characteristic, we will try the next and
            // check again next round
            option_node.characteristic = next_characteristic.cloned();
            // nodes should be with a length of 0
            return Ok(nodes);
        } else {
            panic!("There should not exist a node where some have a characteristic and some don't");
        }
    }

    for boolean in [true, false] {
        current_node_index += 1;

        let mut group = if boolean {
            true_group.clone()
        } else {
            false_group.clone()
        };

        let node = if group.len() == 1 {
            let plant = group
                .pop()
                .expect("Plant will always exist because we check the lenght");

            let plant_node = PlantNode {
                index: current_node_index,
                plant,
            };

            Node::Plant(plant_node)
        } else {
            let option_node = OptionNode {
                index: current_node_index,
                left_node: None,
                right_node: None,
                possibilities: group,
                characteristic: next_characteristic.cloned(),
            };

            Node::Option(option_node)
        };

        nodes.push(node);

        option_node.insert_child_index(current_node_index);
    }

    Ok(nodes)
}

impl Key {
    pub fn new() -> Key {
        Key {
            current_node_index: 0,
            nodes: HashMap::new(),
        }
    }

    pub fn build(&mut self, plants: &[Plant], characteristics: &[Characteristic]) -> Result<()> {
        let initial_option = OptionNode {
            index: self.current_node_index,
            left_node: None,
            right_node: None,
            possibilities: plants.to_vec(),
            characteristic: characteristics.get(0).cloned(),
        };

        self.nodes
            .insert(self.current_node_index, Node::Option(initial_option));

        let mut indexes_to_check: Vec<u32> = Vec::new();

        // start by checking initial index
        indexes_to_check.push(self.current_node_index);

        for (char_idx, _characteristic) in characteristics.iter().enumerate() {
            let mut current_indexes_to_check: VecDeque<u32> = indexes_to_check.clone().into();

            while let Some(current_index) = current_indexes_to_check.pop_front() {
                let next_nodes = {
                    let current_node = self.nodes.get_mut(&current_index).unwrap_or_else(|| {
                        panic!(
                            "Node {} should exist if it was in indexes_to_check",
                            current_index
                        )
                    });

                    let next_characteristic = characteristics.get(char_idx + 1);

                    let option_node = if let Node::Option(option_node) = current_node {
                        option_node
                    } else {
                        continue;
                    };

                    handle_option_node(option_node, next_characteristic, self.current_node_index)?
                };

                // Remove the current index from the indexes to check if it generated new nodes
                if !next_nodes.is_empty() {
                    if let Some(idx) = indexes_to_check
                        .iter()
                        .position(|value| *value == current_index)
                    {
                        indexes_to_check.remove(idx);
                    }
                }

                for node in next_nodes {
                    let new_node_index = node.get_index();

                    self.nodes.insert(new_node_index, node.clone());

                    self.current_node_index += 1;

                    indexes_to_check.push(new_node_index);
                }
            }
        }

        Ok(())
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
            *index += 1;
            let node_index = *index;

            let node_characteristic = option_node.characteristic.as_ref().unwrap();

            writeln!(
                f,
                "{}: {} is {}",
                node_index, node_characteristic.name, node_characteristic.true_option
            )
            .unwrap();

            if let Some(left_node_idx) = option_node.left_node {
                let left_node = nodes.get(&left_node_idx).unwrap();
                pre_order_traversal(left_node, nodes, f, index, node_index + 1);
            }

            for _ in 0..depth {
                write!(f, " ").unwrap();
            }

            writeln!(
                f,
                "{}: {} is {}",
                node_index, node_characteristic.name, node_characteristic.false_option
            )
            .unwrap();

            if let Some(right_node_idx) = option_node.right_node {
                let right_node = nodes.get(&right_node_idx).unwrap();
                pre_order_traversal(right_node, nodes, f, index, node_index + 1);
            }
        }
        Node::Plant(plant_node) => {
            writeln!(f, "{}", plant_node.plant).unwrap();
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut global_index = 0;

        pre_order_traversal(
            self.nodes.get(&0).unwrap(),
            &self.nodes,
            f,
            &mut global_index,
            0,
        );

        Ok(())
    }
}
