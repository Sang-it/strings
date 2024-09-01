use crate::{
    generators::ScannerConfig,
    grammar::{ProductionAttribute, SymbolAttribute, TerminalKind},
    runtime::lexer::{Location, Token},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, marker::PhantomData};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Factor {
    Group(Alternations),
    Repeat(Alternations),
    Optional(Alternations),
    Terminal(
        String,
        TerminalKind,
        Vec<usize>,
        SymbolAttribute,
        Option<UserDefinedTypeName>,
    ),
    NonTerminal(String, SymbolAttribute, Option<UserDefinedTypeName>),
    Identifier(String),
    ScannerSwitch(usize, Location),
    ScannerSwitchPush(usize, Location),
    ScannerSwitchPop(Location),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Alternation(pub Vec<Factor>, pub ProductionAttribute);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Alternations(pub Vec<Alternation>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Production {
    pub lhs: String,
    pub rhs: Alternations,
}

#[derive(Debug, Clone, Default, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserDefinedTypeName(Vec<String>);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum GrammarType {
    #[default]
    LLK,
    LALR1,
}

#[derive(Debug, Clone, Default)]
pub struct Grammar<'t> {
    pub productions: Vec<Production>,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub start_symbol: String,
    pub scanner_configurations: Vec<ScannerConfig>,
    pub user_type_definitions: BTreeMap<String, UserDefinedTypeName>,
    pub grammar_type: GrammarType,
    token_aliases: Vec<(Token<'static>, String)>,
    phantom: PhantomData<&'t str>,
}
