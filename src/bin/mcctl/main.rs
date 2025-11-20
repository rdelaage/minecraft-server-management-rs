use minecraft_rpc::ClientTrait;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(long)]
    url: String,
    #[arg(long)]
    secret: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Allowlist related actions
    Allowlist(AllowlistArgs),
    /// Bans related actions
    Bans(BansArgs),
}

#[derive(Args)]
struct AllowlistArgs {
    #[command(subcommand)]
    command: AllowlistCommand,
}

#[derive(Subcommand)]
enum AllowlistCommand {
    Get,
    Clear,
    Set(AllowlistSetArgs),
    Add(AllowlistAddArgs),
    Remove(AllowlistRemoveArgs),
}

#[derive(Args)]
struct AllowlistSetArgs {
    players: Vec<String>,
}

#[derive(Args)]
struct AllowlistAddArgs {
    players: Vec<String>,
}

#[derive(Args)]
struct AllowlistRemoveArgs {
    players: Vec<String>,
}

#[derive(Args)]
struct BansArgs {
    #[command(subcommand)]
    command: BansCommand,
}

#[derive(Subcommand)]
enum BansCommand {
    Get,
    Clear,
    Set(BansSetArgs),
    Add(BansAddArgs),
    Remove(BansRemoveArgs),
}

#[derive(Args)]
struct BansSetArgs {
    #[arg(long)]
    reason: Option<String>,
    players: Vec<String>,
}

#[derive(Args)]
struct BansAddArgs {
    #[arg(long)]
    reason: Option<String>,
    players: Vec<String>,
}

#[derive(Args)]
struct BansRemoveArgs {
    players: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = minecraft_rpc::new_client(&cli.url, &cli.secret).await?;
    match &cli.command {
        Command::Allowlist(args) => match &args.command {
            AllowlistCommand::Get => {
                let players = client.allowlist_get().await?;
                println!("{players:?}");
            }
            AllowlistCommand::Clear => {
                let players = client.allowlist_clear().await?;
                println!("{players:?}");
            }
            AllowlistCommand::Set(args) => {
                let players = client
                    .allowlist_set(
                        &args
                            .players
                            .iter()
                            .map(|s| s.parse::<minecraft_rpc::Player>().unwrap())
                            .collect::<Vec<minecraft_rpc::Player>>(),
                    )
                    .await?;
                println!("{players:?}");
            }
            AllowlistCommand::Add(args) => {
                let players = client
                    .allowlist_add(
                        &args
                            .players
                            .iter()
                            .map(|s| s.parse::<minecraft_rpc::Player>().unwrap())
                            .collect::<Vec<minecraft_rpc::Player>>(),
                    )
                    .await?;
                println!("{players:?}");
            }
            AllowlistCommand::Remove(args) => {
                let players = client
                    .allowlist_remove(
                        &args
                            .players
                            .iter()
                            .map(|s| s.parse::<minecraft_rpc::Player>().unwrap())
                            .collect::<Vec<minecraft_rpc::Player>>(),
                    )
                    .await?;
                println!("{players:?}");
            }
        },
        Command::Bans(args) => match &args.command {
            BansCommand::Get => {
                let bans = client.bans_get().await?;
                println!("{bans:?}");
            },
            BansCommand::Clear => {
                let bans = client.bans_clear().await?;
                println!("{bans:?}");
            },
            BansCommand::Set(args) => {
                let bans = client
                    .bans_set(
                        &args
                            .players
                            .iter()
                            .map(|p| minecraft_rpc::UserBan::new(p, args.reason.clone()))
                            .collect::<Vec<minecraft_rpc::UserBan>>(),
                    )
                    .await?;
                println!("{bans:?}");
            },
            BansCommand::Add(args) => {
                let bans = client
                    .bans_add(
                        &args
                            .players
                            .iter()
                            .map(|p| minecraft_rpc::UserBan::new(p, args.reason.clone()))
                            .collect::<Vec<minecraft_rpc::UserBan>>(),
                    )
                    .await?;
                println!("{bans:?}");
            },
            BansCommand::Remove(args) => {
                let bans = client
                    .bans_remove(
                        &args
                            .players
                            .iter()
                            .map(|s| s.parse::<minecraft_rpc::Player>().unwrap())
                            .collect::<Vec<minecraft_rpc::Player>>(),
                    )
                    .await?;
                println!("{bans:?}");
            },
        },
    }
    Ok(())
}
