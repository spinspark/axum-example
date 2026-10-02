use crate::config::Config;
use crate::http::{HttpServer, HttpServerConfig};
use crate::sqlite::establish_pool;

mod config;
mod error;
mod http;
mod models;

mod sqlite;
#[cfg(test)]
mod test;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let sqlite = establish_pool(config.database_url()).await?;

    let server_config = HttpServerConfig::new(config.server_port());
    let http_server = HttpServer::new(sqlite, server_config).await?;
    http_server.run().await
}
