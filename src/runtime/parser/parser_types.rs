use crate::runtime::parser::ParseTreeType;
use syntree::Tree;

pub type ParseTree<'t> = Tree<ParseTreeType<'t>, u32, usize>;
