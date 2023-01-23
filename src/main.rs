use {
    dotenv::dotenv,
    bouncer::{config::Configuration, Result},
    tokio::sync::broadcast,
};

#[tokio::main]
async fn main() -> Result<()> {
    let (_signal, shutdown) = broadcast::channel(1);
    dotenv().ok();

    let config = Configuration::new().expect("Failed to load config!");
    bouncer::bootstap(shutdown, config).await
}
