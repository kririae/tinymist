use std::{ops::Deref, sync::Arc};

use nlprule::{rules_filename, tokenizer_filename, Rules, Tokenizer};
use once_cell::sync::Lazy;
use typst::{diag::eco_format, syntax::Span};
use typst_ts_core::TypstDocument;

use super::text_export::MappedSpan;

fn nlp_check(inp: &str) -> Vec<nlprule::types::Suggestion> {
    static TOKENIZER: Lazy<Tokenizer> = Lazy::new(|| {
        let mut tokenizer_bytes: &'static [u8] =
            include_bytes!(concat!(env!("OUT_DIR"), "/", tokenizer_filename!("en")));
        Tokenizer::from_reader(&mut tokenizer_bytes).expect("tokenizer binary is valid")
    });
    static RULES: Lazy<Rules> = Lazy::new(|| {
        let mut rules_bytes: &'static [u8] =
            include_bytes!(concat!(env!("OUT_DIR"), "/", rules_filename!("en")));
        Rules::from_reader(&mut rules_bytes).expect("rules binary is valid")
    });

    RULES.suggest(inp, TOKENIZER.deref())
}

/// Suggestion for change in a text.
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub source: String,
    pub message: String,
    pub span: Option<MappedSpan>,
    pub replacements: Vec<String>,
}

pub fn nlp_check_docs(doc: Arc<TypstDocument>) -> Option<Vec<Suggestion>> {
    let annotated = super::text_export::TextExporter::default()
        .annotate(doc)
        .ok()?;
    let suggestions = nlp_check(&annotated.content);
    let spans = suggestions
        .iter()
        .map(|s| s.span().char().clone())
        .collect::<Vec<_>>();
    let spans = annotated.map_back_spans(spans);
    Some(
        suggestions
            .into_iter()
            .zip(spans)
            .map(|(suggestion, span)| Suggestion {
                source: suggestion.source().to_string(),
                message: suggestion.message().to_string(),
                span,
                replacements: suggestion
                    .replacements()
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
            })
            .collect(),
    )
}

pub fn diag_from_suggestion(suggestion: Suggestion) -> typst::diag::SourceDiagnostic {
    typst::diag::SourceDiagnostic {
        severity: typst::diag::Severity::Warning,
        message: eco_format!("{:?}", suggestion.message),
        span: suggestion
            .span
            .map(|s| s.span.span)
            .unwrap_or(Span::detached()),
        trace: Default::default(),
        hints: Default::default(),
    }
}
