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
    /// IP Bans related actions
    IPBans(IPBansArgs),
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

#[derive(Args)]
struct IPBansArgs {
    #[command(subcommand)]
    command: IPBansCommand,
}

#[derive(Subcommand)]
enum IPBansCommand {
    Get,
    Clear,
    Set(IPBansSetArgs),
    Add(IPBansAddArgs),
    Remove(IPBansRemoveArgs),
}

#[derive(Args)]
struct IPBansSetArgs {
    #[arg(long)]
    reason: Option<String>,
    ips: Vec<String>,
}

#[derive(Args)]
struct IPBansAddArgs {
    #[arg(long)]
    is_ip: bool,
    #[arg(long)]
    reason: Option<String>,
    items: Vec<String>,
}

#[derive(Args)]
struct IPBansRemoveArgs {
    ips: Vec<String>,
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
            }
            BansCommand::Clear => {
                let bans = client.bans_clear().await?;
                println!("{bans:?}");
            }
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
            }
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
            }
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
            }
        },
        Command::IPBans(args) => match &args.command {
            IPBansCommand::Get => {
                let ip_bans = client.ip_bans_get().await?;
                println!("{ip_bans:?}");
            }
            IPBansCommand::Clear => {
                let ip_bans = client.ip_bans_clear().await?;
                println!("{ip_bans:?}");
            }
            IPBansCommand::Set(args) => {
                let ip_bans = client
                    .ip_bans_set(
                        &args
                            .ips
                            .iter()
                            .map(|i| minecraft_rpc::IPBan::new(i, args.reason.clone()))
                            .collect::<Vec<minecraft_rpc::IPBan>>(),
                    )
                    .await?;
                println!("{ip_bans:?}");
            }
            IPBansCommand::Add(args) => {
                let ip_bans = client
                    .ip_bans_add(
                        &args
                            .items
                            .iter()
                            .map(|i| {
                                minecraft_rpc::IncomingIPBan::new(
                                    i,
                                    args.reason.clone(),
                                    args.is_ip,
                                )
                            })
                            .collect::<Vec<minecraft_rpc::IncomingIPBan>>(),
                    )
                    .await?;
                println!("{ip_bans:?}");
            }
            IPBansCommand::Remove(args) => {
                let ip_bans = client.ip_bans_remove(&args.ips).await?;
                println!("{ip_bans:?}");
            }
        },
    }
    Ok(())
}
