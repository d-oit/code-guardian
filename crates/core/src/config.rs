use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub scan_patterns: Vec<String>,
    pub output_formats: Vec<String>,
    pub database_path: String,
    pub max_threads: usize,
    pub cache_size: usize,
    pub batch_size: usize,
    pub max_file_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            scan_patterns: vec!["*.rs".to_string(), "*.toml".to_string()],
            output_formats: vec!["json".to_string()],
            database_path: "data/code-guardian.db".to_string(),
            max_threads: num_cpus::get(),
            cache_size: 50000,
            batch_size: 100,
            max_file_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

pub fn load_config<P: AsRef<Path>>(path: Option<P>) -> anyhow::Result<Config> {
    let mut builder = config::Config::builder();

    // Add default values
    builder = builder.set_default("scan_patterns", vec!["*.rs", "*.toml"])?;
    builder = builder.set_default("output_formats", vec!["json"])?;
    builder = builder.set_default("database_path", "data/code-guardian.db")?;
    builder = builder.set_default("max_threads", num_cpus::get() as i64)?;
    builder = builder.set_default("cache_size", 50000i64)?;
    builder = builder.set_default("batch_size", 100i64)?;
    builder = builder.set_default("max_file_size", (10 * 1024 * 1024) as i64)?;

    // Add file source if provided
    if let Some(path) = path {
        let path = path.as_ref();
        if path.exists() {
            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            match extension {
                "toml" => {
                    builder = builder.add_source(config::File::with_name(path.to_str().unwrap()));
                }
                "json" => {
                    builder = builder.add_source(config::File::with_name(path.to_str().unwrap()));
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Unsupported config file format: {}",
                        extension
                    ))
                }
            }
        }
    }

    let config = builder.build()?;
    let parsed: Config = config.try_deserialize()?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(!config.scan_patterns.is_empty());
        assert!(!config.output_formats.is_empty());
        assert!(!config.database_path.is_empty());
        assert!(config.max_threads > 0);
        assert_eq!(config.cache_size, 50000);
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.max_file_size, 10 * 1024 * 1024);
    }

    #[test]
    fn test_load_config_toml() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let toml_content = r#"
scan_patterns = ["*.rs", "*.py"]
output_formats = ["json", "csv"]
database_path = "test.db"
max_threads = 4
cache_size = 100000
batch_size = 200
max_file_size = 20971520
"#;
        fs::write(&config_path, toml_content).unwrap();

        let config = load_config(Some(&config_path)).unwrap();
        assert_eq!(config.scan_patterns, vec!["*.rs", "*.py"]);
        assert_eq!(config.output_formats, vec!["json", "csv"]);
        assert_eq!(config.database_path, "test.db");
        assert_eq!(config.max_threads, 4);
        assert_eq!(config.cache_size, 100_000);
        assert_eq!(config.batch_size, 200);
        assert_eq!(config.max_file_size, 20_971_520);
    }

    #[test]
    fn test_load_config_json() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        let json_content = r#"{
"scan_patterns": ["*.js", "*.ts"],
"output_formats": ["html"],
"database_path": "data.db",
"max_threads": 8,
"cache_size": 75000,
"batch_size": 150,
"max_file_size": 15728640
}"#;
        fs::write(&config_path, json_content).unwrap();

        let config = load_config(Some(&config_path)).unwrap();
        assert_eq!(config.scan_patterns, vec!["*.js", "*.ts"]);
        assert_eq!(config.output_formats, vec!["html"]);
        assert_eq!(config.database_path, "data.db");
        assert_eq!(config.max_threads, 8);
        assert_eq!(config.cache_size, 75000);
        assert_eq!(config.batch_size, 150);
        assert_eq!(config.max_file_size, 15_728_640);
    }

    #[test]
    fn test_load_config_no_file() {
        let config = load_config::<&str>(None).unwrap();
        let default = Config::default();
        assert_eq!(config.scan_patterns, default.scan_patterns);
        assert_eq!(config.output_formats, default.output_formats);
        assert_eq!(config.database_path, default.database_path);
        assert_eq!(config.max_threads, default.max_threads);
        assert_eq!(config.cache_size, default.cache_size);
        assert_eq!(config.batch_size, default.batch_size);
        assert_eq!(config.max_file_size, default.max_file_size);
    }

    #[test]
    fn test_load_config_unsupported_format() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.txt");
        fs::write(&config_path, "invalid").unwrap();

        let result = load_config(Some(&config_path));
        assert!(result.is_err());
    }
}
