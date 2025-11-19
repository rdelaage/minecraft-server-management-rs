use std::str::FromStr;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::{HeaderMap, HeaderValue, WsClient, WsClientBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UntypedGameRule {
    value: String,
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingIPBan {
    reason: String,
    expires: String,
    ip: String,
    source: String,
    player: Player,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemMessage {
    receiveing_players: Vec<Player>,
    overlay: bool,
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KickPlayer {
    player: Player,
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IPBan {
    reason: String,
    expires: String,
    ip: String,
    source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypedGameRule {
    r#type: String,
    value: String,
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserBan {
    reason: String,
    expires: Option<String>,
    source: Option<String>,
    player: Player,
}

impl UserBan {
    // was not able to understand expires and source (no doc)
    pub fn new(player_name: &str, reason: &str) -> Self {
        UserBan {
            reason: reason.to_string(),
            expires: None,
            source: None,
            player: player_name.parse().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    translatable: String,
    translatable_params: Vec<String>,
    literal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    protocol: i64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerState {
    players: Vec<Player>,
    started: bool,
    version: Version,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operator {
    permission_level: i64,
    bypasses_player_limit: bool,
    player: Player,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub id: Option<String>,
}

impl FromStr for Player {
    type Err = Error;
    // Enable "player".parse(). Should never fail, unwrap is safe
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Player {
            name: s.to_string(),
            id: None,
        })
    }
}

#[derive(Debug)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinecraftRpcError")
    }
}

impl std::error::Error for Error {}

pub trait ClientTrait {
    /*************
     * ALLOWLIST *
     *************/
    fn allowlist_get(
        &self,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Player>>> + Send;
    fn allowlist_set(
        &self,
        players: &[Player],
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Player>>> + Send;
    fn allowlist_add(
        &self,
        players: &[Player],
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Player>>> + Send;
    fn allowlist_remove(
        &self,
        players: &[Player],
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Player>>> + Send;
    fn allowlist_clear(
        &self,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Player>>> + Send;

    /********
     * BANS *
     ********/
    fn bans_get(&self) -> impl std::future::Future<Output = anyhow::Result<Vec<UserBan>>> + Send;
    fn bans_set(
        &self,
        bans: &[UserBan],
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<UserBan>>> + Send;
    fn bans_add(
        &self,
        bans: &[UserBan],
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<UserBan>>> + Send;
    fn bans_remove(
        &self,
        players: &[Player],
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<UserBan>>> + Send;
    fn bans_clear(&self) -> impl std::future::Future<Output = anyhow::Result<Vec<UserBan>>> + Send;
}

struct Client {
    ws_client: WsClient,
}

pub async fn new_client(url: &str, secret: &str) -> anyhow::Result<impl ClientTrait> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {secret}"))?,
    );
    let client = WsClientBuilder::new()
        .set_headers(headers)
        .build(url)
        .await?;
    Ok(Client { ws_client: client })
}

impl ClientTrait for Client {
    /*************
     * ALLOWLIST *
     *************/
    async fn allowlist_get(&self) -> anyhow::Result<Vec<Player>> {
        let players: Vec<Player> = self
            .ws_client
            .request("minecraft:allowlist", rpc_params![])
            .await?;
        Ok(players)
    }
    async fn allowlist_set(&self, players: &[Player]) -> anyhow::Result<Vec<Player>> {
        let players: Vec<Player> = self
            .ws_client
            .request("minecraft:allowlist/set", rpc_params![players])
            .await?;
        Ok(players)
    }
    async fn allowlist_add(&self, players: &[Player]) -> anyhow::Result<Vec<Player>> {
        let players: Vec<Player> = self
            .ws_client
            .request("minecraft:allowlist/add", rpc_params![players])
            .await?;
        Ok(players)
    }
    async fn allowlist_remove(&self, players: &[Player]) -> anyhow::Result<Vec<Player>> {
        let players: Vec<Player> = self
            .ws_client
            .request("minecraft:allowlist/remove", rpc_params![players])
            .await?;
        Ok(players)
    }
    async fn allowlist_clear(&self) -> anyhow::Result<Vec<Player>> {
        let players: Vec<Player> = self
            .ws_client
            .request("minecraft:allowlist/clear", rpc_params![])
            .await?;
        Ok(players)
    }

    /********
     * BANS *
     ********/
    async fn bans_get(&self) -> anyhow::Result<Vec<UserBan>> {
        let bans: Vec<UserBan> = self
            .ws_client
            .request("minecraft:bans", rpc_params![])
            .await?;
        Ok(bans)
    }
    async fn bans_set(&self, bans: &[UserBan]) -> anyhow::Result<Vec<UserBan>> {
        let bans: Vec<UserBan> = self
            .ws_client
            .request("minecraft:bans/set", rpc_params![bans])
            .await?;
        Ok(bans)
    }
    async fn bans_add(&self, bans: &[UserBan]) -> anyhow::Result<Vec<UserBan>> {
        let bans: Vec<UserBan> = self
            .ws_client
            .request("minecraft:bans/add", rpc_params![bans])
            .await?;
        Ok(bans)
    }
    async fn bans_remove(&self, players: &[Player]) -> anyhow::Result<Vec<UserBan>> {
        let bans: Vec<UserBan> = self
            .ws_client
            .request("minecraft:bans/remove", rpc_params![players])
            .await?;
        Ok(bans)
    }
    async fn bans_clear(&self) -> anyhow::Result<Vec<UserBan>> {
        let bans: Vec<UserBan> = self
            .ws_client
            .request("minecraft:bans/clear", rpc_params![])
            .await?;
        Ok(bans)
    }
}
