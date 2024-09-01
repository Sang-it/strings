use crate::runtime::lexer::Token;

#[derive(Debug, Clone)]
pub enum ParseTreeType<'t> {
    T(Token<'t>),
    N(&'static str),
}
