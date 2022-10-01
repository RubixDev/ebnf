use std::fmt::Debug;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Configuration {
    pub line_width: usize,
    pub newline_kind: NewlineKind,
    pub quote_style: QuoteStyle,
    pub ignore_rule_comment_text: String,
    pub mutliline_comment_indent: usize,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            line_width: 100,
            newline_kind: NewlineKind::Unix,
            quote_style: QuoteStyle::Single,
            ignore_rule_comment_text: "ebnf-fmt ignore".to_string(),
            mutliline_comment_indent: 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "strum", derive(strum::EnumString))]
pub enum QuoteStyle {
    Single,
    Double,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "strum", derive(strum::EnumString))]
pub enum NewlineKind {
    Unix,
    Windows,
}
