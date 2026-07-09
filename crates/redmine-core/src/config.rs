use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(name = "redmine-mcp", about = "MCP server for Redmine")]
pub struct Config {
    #[arg(
        long,
        env = "REDMINE_URL",
        help = "Redmine instance URL (e.g. https://redmine.example.com)"
    )]
    pub redmine_url: String,

    #[arg(
        long,
        env = "REDMINE_API_KEY",
        help = "Redmine API key"
    )]
    pub redmine_api_key: String,
}

impl Config {
    pub fn parse() -> Self {
        let _ = dotenvy::dotenv();
        <Self as Parser>::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_debug() {
        let config = Config {
            redmine_url: "https://redmine.example.com".into(),
            redmine_api_key: "test-key-123".into(),
        };
        assert!(format!("{config:?}").contains("redmine_url"));
        assert!(format!("{config:?}").contains("redmine_api_key"));
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
}
