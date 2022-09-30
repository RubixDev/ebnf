#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Configuration {
    pub line_width: usize,
    pub newline_kind: NewlineKind,
    pub quote_style: QuoteStyle,
    pub ignore_node_comment_text: String,
    pub mutliline_comment_indent: usize,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QuoteStyle {
    Single,
    Double,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NewlineKind {
    Unix,
    Windows,
}
