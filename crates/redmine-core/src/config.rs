use clap::Parser;
use std::fmt;

/// CLI and environment configuration for the Redmine MCP server.
///
/// Fields can be set via command-line arguments or environment variables.
/// Supports `.env` file loading via [`parse`](Self::parse).
#[derive(Clone, Parser)]
#[command(name = "redmine-mcp", about = "MCP server for Redmine")]
pub struct Config {
    /// Redmine instance URL (e.g. `https://redmine.example.com`).
    #[arg(
        long,
        env = "REDMINE_URL",
        help = "Redmine instance URL (e.g. https://redmine.example.com)"
    )]
    pub redmine_url: String,

    /// Redmine API key for authentication.
    #[arg(
        long,
        env = "REDMINE_API_KEY",
        help = "Redmine API key"
    )]
    pub redmine_api_key: String,
}

/// Custom [`Debug`] implementation that redacts the API key.
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("redmine_url", &self.redmine_url)
            .field("redmine_api_key", &"***REDACTED***")
            .finish()
    }
}

impl Config {
    /// Parse configuration from CLI args, falling back to environment variables.
    ///
    /// Loads `.env` file if present, then parses `clap` arguments.
    pub fn parse() -> Self {
        let _ = dotenvy::dotenv();
        <Self as Parser>::parse()
    }

    /// Parse configuration from a custom iterator of arguments (useful for testing).
    pub fn parse_from<I>(args: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<std::ffi::OsString> + Clone,
    {
        <Self as Parser>::parse_from(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_debug_does_not_leak_api_key() {
        let config = Config {
            redmine_url: "https://redmine.example.com".into(),
            redmine_api_key: "my-secret-key-12345".into(),
        };
        let debug_str = format!("{config:?}");
        // Should contain the field names
        assert!(debug_str.contains("redmine_url"));
        assert!(debug_str.contains("redmine_api_key"));
        // SECURITY: Debug should NOT contain the actual API key value
        assert!(!debug_str.contains("my-secret-key-12345"), "API key value leaked in Debug output!");
        // Should contain a redacted marker instead
        assert!(debug_str.contains("REDACTED"), "Debug output should indicate the API key is redacted");
    }

    #[test]
    fn test_config_clone() {
        let a = Config {
            redmine_url: "https://redmine.example.com".into(),
            redmine_api_key: "test-key-123".into(),
        };
        let b = a.clone();
        assert_eq!(a.redmine_url, b.redmine_url);
        assert_eq!(a.redmine_api_key, b.redmine_api_key);
    }

    #[test]
    fn test_config_parse_custom_args() {
        let config = Config::parse_from([
            "test",
            "--redmine-url", "https://redmine.example.com",
            "--redmine-api-key", "secret-key-123",
        ]);
        assert_eq!(config.redmine_url, "https://redmine.example.com");
        assert_eq!(config.redmine_api_key, "secret-key-123");
    }

    #[test]
    fn test_config_redmine_url_format() {
        let config = Config {
            redmine_url: "https://redmine.example.com".into(),
            redmine_api_key: "key".into(),
        };
        assert!(config.redmine_url.starts_with("http"));
    }
}
