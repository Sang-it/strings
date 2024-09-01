#[derive(Debug, Default, Clone)]
pub struct Cfg {
    /// Start symbol of the grammar
    pub st: String,
    /// Set of productions
    pub pr: Vec<Pr>,
}

