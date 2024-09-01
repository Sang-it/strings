use crate::generators::GrammarConfig;
use crate::parser::grammar::Grammar;
use crate::runtime::{ParseTree, Result};
use crate::MAX_K;
use std::path::{Path, PathBuf};

pub const DEFAULT_MAX_LOOKAHEAD: usize = 5;
pub const DEFAULT_MODULE_NAME: &str = "grammar";
pub const DEFAULT_USER_TYPE_NAME: &str = "Grammar";

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BuilderError {
    #[error("Missing an input grammar file")]
    MissingGrammarFile,
    #[error("No parser output file specified")]
    MissingParserOutputFile,
    #[error("No action output file specified")]
    MissingActionOutputFile,
    #[error("Maximum lookahead is {}", MAX_K)]
    LookaheadTooLarge,
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntermediateGrammar {
    Untransformed,
    Transformed,
}
impl IntermediateGrammar {
    pub const LAST: IntermediateGrammar = IntermediateGrammar::Transformed;
}

#[allow(unused_variables)]
pub trait BuildListener {
    fn on_initial_grammar_parse(
        &mut self,
        syntax_tree: &ParseTree<'_>,
        grammar: &Grammar,
    ) -> Result<()> {
        Ok(())
    }
    fn on_intermediate_grammar(
        &mut self,
        stage: IntermediateGrammar,
        config: &GrammarConfig,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Default)]
struct MaybeBuildListener<'l>(Option<&'l mut dyn BuildListener>);
impl<'l> BuildListener for MaybeBuildListener<'l> {
    fn on_initial_grammar_parse(
        &mut self,
        syntax_tree: &ParseTree<'_>,
        grammar: &Grammar,
    ) -> Result<()> {
        if let Some(ref mut inner) = self.0 {
            inner.on_initial_grammar_parse(syntax_tree, grammar)
        } else {
            Ok(())
        }
    }

    fn on_intermediate_grammar(
        &mut self,
        stage: IntermediateGrammar,
        config: &GrammarConfig,
    ) -> Result<()> {
        if let Some(ref mut inner) = self.0 {
            inner.on_intermediate_grammar(stage, config)
        } else {
            Ok(())
        }
    }
}

#[allow(unused_variables)]
#[derive(Clone)]
pub struct Builder {
    output_dir: PathBuf,
    grammar_file: Option<PathBuf>,
    expanded_grammar_output_file: Option<PathBuf>,
    parser_output_file: Option<PathBuf>,
    actions_output_file: Option<PathBuf>,
    user_type_name: String,
    module_name: String,
    max_lookahead: usize,
    auto_generate: bool,
}

impl Builder {
    pub fn with_explicit_output_dir(output: impl AsRef<Path>) -> Self {
        Builder {
            output_dir: PathBuf::from(output.as_ref()),
            grammar_file: None,
            max_lookahead: DEFAULT_MAX_LOOKAHEAD,
            module_name: String::from(DEFAULT_MODULE_NAME),
            user_type_name: String::from(DEFAULT_USER_TYPE_NAME),
            parser_output_file: None,
            actions_output_file: None,
            expanded_grammar_output_file: None,
            auto_generate: false,
        }
    }

    pub fn grammar_file(&mut self, grammar: impl AsRef<Path>) -> &mut Self {
        self.grammar_file = Some(PathBuf::from(grammar.as_ref()));
        self
    }

    pub fn expanded_grammar_output_file(&mut self, p: impl AsRef<Path>) -> &mut Self {
        self.expanded_grammar_output_file = Some(self.resolve_output_path(p));
        self
    }

    pub fn parser_output_file(&mut self, p: impl AsRef<Path>) -> &mut Self {
        self.parser_output_file = Some(self.resolve_output_path(p));
        self
    }

    pub fn actions_output_file(&mut self, p: impl AsRef<Path>) -> &mut Self {
        self.actions_output_file = Some(self.resolve_output_path(p));
        self
    }

    pub fn enable_auto_generation(&mut self) -> &mut Self {
        self.auto_generate = true;
        self
    }

    pub fn user_type_name(&mut self, name: &str) -> &mut Self {
        self.user_type_name = name.into();
        self
    }

    pub fn user_trait_module_name(&mut self, name: &str) -> &mut Self {
        self.module_name = name.into();
        self
    }

    pub fn begin_generation_with<'l>(
        &mut self,
        listener: Option<&'l mut dyn BuildListener>,
    ) -> std::result::Result<GrammarGenerator<'l>, BuilderError> {
        let grammar_file = self
            .grammar_file
            .as_ref()
            .ok_or(BuilderError::MissingGrammarFile)?
            .clone();

        Ok(GrammarGenerator {
            listener: MaybeBuildListener(listener),
            grammar_file,
            builder: self.clone(),
            state: None,
            grammar_config: None,
            lookahead_dfa_s: None,
            parse_table: None,
        })
    }

    pub fn generate_parser(&mut self) -> Result<()> {
        self.begin_generation_with(None)
            .map_err(|e| panic!("Misconfigured parol generation: {}", e))?
            .generate_parser()
    }

    fn resolve_output_path(&self, p: impl AsRef<Path>) -> PathBuf {
        self.output_dir.join(p)
    }
}

pub struct GrammarGenerator<'l> {
    listener: MaybeBuildListener<'l>,
    pub(crate) grammar_file: PathBuf,
    builder: Builder,
    state: Option<State>,
    pub(crate) grammar_config: Option<GrammarConfig>,
    lookahead_dfa_s: Option<BTreeMap<String, LookaheadDFA>>,
    parse_table: Option<LRParseTable>,
}
