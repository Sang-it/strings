use crate::grammar::Pr;

#[derive(Debug, Default, Clone)]
pub struct Cfg {
    pub st: String,
    pub pr: Vec<Pr>,
}
