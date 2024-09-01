use crate::{generators::ScannerConfig, grammar::Cfg, parser::grammar::GrammarType};

#[derive(Debug, Clone, Default)]
pub struct GrammarConfig {
    pub non_terminals: Vec<String>,
    pub cfg: Cfg,
    pub grammar_type: GrammarType,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub user_type_defs: Vec<(String, String)>,
    pub scanner_configurations: Vec<ScannerConfig>,
    pub lookahead_size: usize,
}
