use dprint_core::configuration::{
    self, ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult,
};

use crate::configuration::Configuration;

macro_rules! resolve_config {
    ($config:ident, $global_config:ident, $diagnostics:ident, $default_config:ident; $($field:ident $($global:ident)?),* $(,)?) => {
        Configuration {$(
            $field: configuration::get_value(
                &mut $config,
                heck_but_macros::stringify_mixed_case!($field),
                resolve_config!(@global $global_config, $default_config, $($global)? $field),
                &mut $diagnostics,
            ),
        )*}
    };
    (@global $global_config:ident, $default_config:ident, global $field:ident) => {
        $global_config.$field.unwrap_or($default_config.$field)
    };
    (@global $global_config:ident, $default_config:ident, $field:ident) => {
        $default_config.$field
    };
}

pub fn resolve_config(
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    let mut config = config;
    let mut diagnostics = vec![];

    let default_config = Configuration::default();
    let resolved_config = resolve_config!(
        config, global_config, diagnostics, default_config;
        line_width    global,
        indent_width  global,
        new_line_kind global,
        quote_style,
        ignore_rule_comment_text,
        multiline_comments_markdown,
    );

    diagnostics.extend(configuration::get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        diagnostics,
        config: resolved_config,
    }
}
