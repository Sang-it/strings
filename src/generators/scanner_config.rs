use crate::runtime::lexer::TerminalIndex;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ScannerConfig {
    pub scanner_name: String,
    pub scanner_state: usize,
    pub line_comments: Vec<String>,
    pub block_comments: Vec<(String, String)>,
    pub auto_newline: bool,
    pub auto_ws: bool,
    pub transitions: Vec<(TerminalIndex, usize)>,
}
