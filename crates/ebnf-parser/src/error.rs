use std::borrow::Cow;

use thiserror::Error;

use crate::span::Span;

#[derive(Debug, Clone, Error)]
#[error("invalid syntax at {span}: {message}")]
pub struct SyntaxError {
    pub span: Span,
    pub message: String,
}

impl SyntaxError {
    pub(crate) fn new(span: Span, message: Cow<str>) -> Self {
        Self {
            span,
            message: message.into_owned(),
        }
    }
}
