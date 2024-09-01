use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SymbolAttribute {
    None,
    RepetitionAnchor,
    Option,
    Clipped,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProductionAttribute {
    None,
    CollectionStart,
    AddToCollection,
    OptionalSome,
    OptionalNone,
}
