use crate::runtime::lexer::{Location, TerminalIndex, TokenNumber};
use std::borrow::Cow;

#[derive(Debug, Clone, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Token<'t> {
    pub(crate) text: Cow<'t, str>,
    pub token_type: TerminalIndex,
    pub location: Location,
    pub token_number: TokenNumber,
}
