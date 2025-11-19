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
    expires: String,
    source: String,
    player: Player,
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
    name: String,
    id: String,
}

pub trait ClientTrait {
    fn allowlist_get(
        &self,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Player>>> + Send;
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
    async fn allowlist_get(&self) -> anyhow::Result<Vec<Player>> {
        let players: Vec<Player> = self
            .ws_client
            .request("minecraft:allowlist", rpc_params![])
            .await?;
        Ok(players)
    }
}
