use crate::grammar::{ProductionAttribute, Symbol};

pub type Rhs = Vec<Symbol>;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pr(pub Symbol, pub Rhs, pub ProductionAttribute);
