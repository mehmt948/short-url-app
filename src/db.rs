use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;

pub async fn get_client() -> Result<Client, Box<dyn Error>> {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    Ok(client)
}