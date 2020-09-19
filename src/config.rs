use anyhow::{Context, Result};
use std::env;

const KEYWORDS_ENV_VAR_KEY: &str = "REDACT_KEYWORDS";

#[derive(Eq, PartialEq, Debug)]
pub struct Config {
    keywords: Vec<String>,
}

impl Config {
    pub fn from_env_var() -> Result<Self> {
        let keywords = env::var(KEYWORDS_ENV_VAR_KEY).with_context(|| {
            format!(
                "Failed to get configuration keywords from env var {}",
                KEYWORDS_ENV_VAR_KEY
            )
        })?;
        let keywords: Vec<String> = serde_json::from_str(&keywords).with_context(|| {
            format!(
                "Failed to parse configuration keywords. Expecting {} env var in JSON array format.",
                KEYWORDS_ENV_VAR_KEY
            )
        })?;
        let config = Config { keywords };
        Ok(config)
    }

    pub fn keywords(&self) -> &[String] {
        &self.keywords
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_env_var() {
        env::set_var("REDACT_KEYWORDS", r#"["hello", "world", "lorem ipsum"]"#);
        let config = Config::from_env_var().unwrap();
        let expected_keywords = &["hello", "world", "lorem ipsum"];
        assert_eq!(expected_keywords, config.keywords());
    }
}
