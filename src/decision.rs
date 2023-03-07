use core::fmt::Debug;
use dyn_clone::DynClone;

pub trait Decision: DynClone {
    fn decide(&self) -> bool;
}

dyn_clone::clone_trait_object!(Decision);

impl Debug for dyn Decision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decision")
    }
}

#[derive(Debug, Clone)]
pub enum Arrangement {
    Opposite,
    Alternate,
}

impl Arrangement {
    pub fn show_option(decision: bool) -> String {
        match decision {
            true => String::from("opposite"),
            false => String::from("alternate"),
        }
    }
}

impl Decision for Arrangement {
    fn decide(&self) -> bool {
        match self {
            Arrangement::Opposite => true,
            Arrangement::Alternate => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LeafType {
    Compound,
    Simple,
}

impl Decision for LeafType {
    fn decide(&self) -> bool {
        match self {
            LeafType::Compound => true,
            LeafType::Simple => false,
        }
    }
}

impl LeafType {
    pub fn show_option(decision: bool) -> String {
        match decision {
            true => String::from("compound"),
            false => String::from("simple"),
        }
    }
}

pub enum Characteristic {
    Arrangement(Arrangement),
    LeafType(LeafType),
}

impl Characteristic {
    pub fn show_option(char_string: &str, decision: bool) -> String {
        if char_string == "arrangement" {
            Arrangement::show_option(decision)
        } else if char_string == "leaf_type" {
            LeafType::show_option(decision)
        } else {
            String::from("unknown")
        }
    }
}
