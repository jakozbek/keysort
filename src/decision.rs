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
