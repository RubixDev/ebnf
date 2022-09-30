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
    pub max_count_for_inline_definition_list: usize,
    pub min_terminals_percent_for_inline_definition_list: Percentage,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            line_width: 100,
            newline_kind: NewlineKind::Unix,
            quote_style: QuoteStyle::Single,
            ignore_rule_comment_text: "ebnf-fmt ignore".to_string(),
            mutliline_comment_indent: 2,
            max_count_for_inline_definition_list: 3,
            min_terminals_percent_for_inline_definition_list: Percentage(90),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QuoteStyle {
    Single,
    Double,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NewlineKind {
    Unix,
    Windows,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Percentage(u8);

impl Percentage {
    pub fn new(percentage: u8) -> Self {
        if percentage > 100 {
            panic!("Tried to create Percentage bigger than 100: {}", percentage);
        }
        Self(percentage)
    }

    pub fn get(&self) -> u8 {
        self.0
    }

    pub fn get_f64(&self) -> f64 {
        self.0 as f64 / 100.0
    }
}

impl Debug for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Percentage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PercentageVisitor;

        impl<'de> serde::de::Visitor<'de> for PercentageVisitor {
            type Value = Percentage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer between 0 and 100")
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    0..=100 => Ok(Percentage(v)),
                    _ => Err(E::custom(format!("Percentage out of range: {}", v))),
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    0..=100 => Ok(Percentage(v as u8)),
                    _ => Err(E::custom(format!("Percentage out of range: {}", v))),
                }
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    0..=100 => Ok(Percentage(v as u8)),
                    _ => Err(E::custom(format!("Percentage out of range: {}", v))),
                }
            }
        }

        deserializer.deserialize_u8(PercentageVisitor)
    }
}
