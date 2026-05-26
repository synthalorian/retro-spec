use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct RetroSpecConfig {
    pub repo: Option<String>,
    pub theme: Option<String>,
    pub screenshot: Option<String>,
    pub export: Option<String>,
    pub duration: Option<f32>,
    pub resolution: Option<String>,
    pub windowed: Option<bool>,
}

pub fn load_config(path: &std::path::Path) -> RetroSpecConfig {
    let config_path = if path.exists() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_default()
            .join("retro-spec.toml")
    };

    if config_path.exists() {
        std::fs::read_to_string(&config_path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        RetroSpecConfig::default()
    }
}
