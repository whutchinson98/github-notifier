#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub github_username: String,
    pub github_token: String,
}

impl Config {
    pub fn load_from_file(file_path: &str) -> Self {
        let contents = std::fs::read_to_string(file_path).unwrap();
        let config: Self = toml::from_str(&contents).unwrap();
        config
    }
}
