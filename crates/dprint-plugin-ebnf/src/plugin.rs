use std::path::Path;

use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult},
    plugins::{FormatResult, PluginInfo, SyncPluginHandler},
};

use crate::{configuration::Configuration, resolve_config::resolve_config};

pub struct EbnfPluginHandler;

impl SyncPluginHandler<Configuration> for EbnfPluginHandler {
    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        resolve_config(config, global_config)
    }

    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "ebnf".to_string(),
            file_extensions: vec!["ebnf".to_string()],
            file_names: vec![],
            help_url: concat!(
                env!("CARGO_PKG_REPOSITORY"),
                "/crates/dprint-plugin-ebnf#readme"
            )
            .to_string(),
            config_schema_url: "".to_string(),
            update_url: Some("https://plugins.dprint.dev/RubixDev/ebnf/latest.json".to_string()),
        }
    }

    fn license_text(&mut self) -> String {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE")).into()
    }

    fn format(
        &mut self,
        _file_path: &Path,
        file_text: &str,
        config: &Configuration,
        mut format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> FormatResult,
    ) -> FormatResult {
        let fmt_config = config.to_fmt_config(file_text);

        let result =
            ebnf_fmt::format_code_with_comment_formatter(file_text, &fmt_config, move |text| {
                let mut md_config = ConfigKeyMap::new();
                md_config.insert(
                    "lineWidth".into(),
                    ((fmt_config.line_width - fmt_config.mutliline_comment_indent) as i32).into(),
                );
                match format_with_host(Path::new("file.md"), text.clone(), &md_config) {
                    Ok(Some(output)) => output,
                    _ => text,
                }
            })?;
        if result == file_text {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(EbnfPluginHandler, EbnfPluginHandler);
