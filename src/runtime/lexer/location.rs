use std::{path::PathBuf, sync::Arc};

use derive_builder::Builder;

#[derive(Builder, Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub length: u32,
    #[builder(default)]
    pub scanner_switch_pos: usize,
    pub offset: usize,
    pub file_name: Arc<PathBuf>,
}
