use minecraft_rpc::ClientTrait;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    url: String,

    #[arg(long)]
    secret: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = minecraft_rpc::new_client(&args.url, &args.secret).await?;
    let players = client.allowlist_get().await?;
    println!("{players:?}");
    Ok(())
}
