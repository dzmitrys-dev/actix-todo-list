use config::ConfigError;
use deadpool_postgres::Pool;
use serde::Deserialize;
use slog::{
    Logger,
    Drain
};

#[derive(Clone, Deserialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }

    pub fn configure_log() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, slog::o!("v" => env!("CARGO_PKG_VERSION")))
    }

    pub fn configure_pool(&self) -> Pool {
        self.pg.create_pool(None, tokio_postgres::NoTls).unwrap()
    }
}