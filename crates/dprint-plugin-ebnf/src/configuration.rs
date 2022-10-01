use dprint_core::configuration::{self, NewLineKind};
use ebnf_fmt::configuration::{NewlineKind, QuoteStyle};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub line_width: u32,
    pub indent_width: u8,
    pub new_line_kind: NewLineKind,

    pub quote_style: QuoteStyle,
    pub ignore_rule_comment_text: String,
    pub multiline_comments_markdown: bool,
}

impl Configuration {
    pub fn to_fmt_config(&self, text: &str) -> ebnf_fmt::Configuration {
        ebnf_fmt::Configuration {
            line_width: self.line_width as usize,
            newline_kind: match configuration::resolve_new_line_kind(text, self.new_line_kind) {
                "\r\n" => NewlineKind::Windows,
                "\n" => NewlineKind::Unix,
                // Fall back to \n in case upstream function changes
                _ => NewlineKind::Unix,
            },
            quote_style: self.quote_style,
            ignore_rule_comment_text: self.ignore_rule_comment_text.clone(),
            mutliline_comment_indent: self.indent_width as usize,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let ebnf_fmt_default = ebnf_fmt::Configuration::default();
        Self {
            line_width: ebnf_fmt_default.line_width as u32,
            indent_width: ebnf_fmt_default.mutliline_comment_indent as u8,
            new_line_kind: match ebnf_fmt_default.newline_kind {
                NewlineKind::Unix => NewLineKind::LineFeed,
                NewlineKind::Windows => NewLineKind::CarriageReturnLineFeed,
            },
            quote_style: ebnf_fmt_default.quote_style,
            ignore_rule_comment_text: "dprint-ignore".to_string(),
            multiline_comments_markdown: true,
        }
    }
}
