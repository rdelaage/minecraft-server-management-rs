use clap::Parser;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::{HeaderMap, HeaderValue, WsClientBuilder};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    url: String,

    #[arg(long)]
    secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    name: String,
    id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let token = format!("Bearer {}", args.secret);
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&token)?,
    );
    let client = WsClientBuilder::new()
        .set_headers(headers)
        .build(&args.url)
        .await?;
    let players: Vec<Player> = client
        .request("minecraft:allowlist", rpc_params![])
        .await?;
    println!("{:?}", players);
    Ok(())
}
