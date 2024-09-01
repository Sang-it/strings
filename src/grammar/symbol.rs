use crate::grammar::SymbolAttribute;
use crate::{parser::grammar::UserDefinedTypeName, runtime::parser::ScannerIndex};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Symbol {
    N(String, SymbolAttribute, Option<UserDefinedTypeName>),
    T(Terminal),
    S(ScannerIndex),
    Push(ScannerIndex),
    Pop,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Terminal {
    Trm(
        String,
        TerminalKind,
        Vec<usize>,
        SymbolAttribute,
        Option<UserDefinedTypeName>,
    ),
    Eps,
    End,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TerminalKind {
    Legacy,
    Regex,
    Raw,
}
