use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("{message}")]
pub(crate) struct GenericError {
    message: String,
    #[label]
    label: SourceSpan,
}

impl GenericError {
    pub(crate) fn new(message: impl Into<String>, label: impl Into<SourceSpan>) -> Self {
        Self {
            message: message.into(),
            label: label.into(),
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("failed to parse input")]
pub(crate) struct ParseError {
    #[source_code]
    source_code: String,
    #[related]
    errors: Vec<GenericError>,
}

impl ParseError {
    pub(crate) fn new(
        source: impl Into<String>,
        errors: impl Iterator<Item = GenericError>,
    ) -> Self {
        Self {
            source_code: source.into(),
            errors: errors.collect(),
        }
    }
}
