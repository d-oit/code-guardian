use anyhow::Result;

use crate::cli_definitions::StackPreset;
use crate::production_handlers::handle_lang_scan;

/// Handle stack preset commands by mapping them to appropriate language configurations
pub fn handle_stack_preset(preset: StackPreset) -> Result<()> {
    match preset {
        StackPreset::Web { path, production } => {
            let languages = vec![
                "js".to_string(),
                "ts".to_string(),
                "jsx".to_string(),
                "tsx".to_string(),
                "vue".to_string(),
                "svelte".to_string(),
            ];
            handle_lang_scan(languages, path, "text".to_string(), production)
        }
        StackPreset::Backend { path, production } => {
            let languages = vec![
                "py".to_string(),
                "java".to_string(),
                "go".to_string(),
                "cs".to_string(),
                "php".to_string(),
                "rb".to_string(),
            ];
            handle_lang_scan(languages, path, "text".to_string(), production)
        }
        StackPreset::Fullstack { path, production } => {
            let languages = vec![
                "js".to_string(),
                "ts".to_string(),
                "py".to_string(),
                "java".to_string(),
                "go".to_string(),
                "rs".to_string(),
            ];
            handle_lang_scan(languages, path, "text".to_string(), production)
        }
        StackPreset::Mobile { path, production } => {
            let languages = vec![
                "js".to_string(),
                "ts".to_string(),
                "swift".to_string(),
                "kt".to_string(),
                "dart".to_string(),
            ];
            handle_lang_scan(languages, path, "text".to_string(), production)
        }
        StackPreset::Systems { path, production } => {
            let languages = vec![
                "rs".to_string(),
                "cpp".to_string(),
                "c".to_string(),
                "go".to_string(),
            ];
            handle_lang_scan(languages, path, "text".to_string(), production)
        }
    }
}
