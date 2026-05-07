use crate::probe::inventory_packets::PlayerInventorySlot;
use crate::probe::play_bootstrap::{complete_configuration, complete_play_bootstrap};
use crate::probe::validation::{LoginPacket, PositionPacket, validate_login_success};
use crate::probe::vitals_packets::HealthState;
use crate::protocol::types::{LoginStart, NextState};
use crate::protocol::{codec, ids};
use tokio::net::TcpStream;
use uuid::Uuid;

pub(super) struct PlayClient {
    pub stream: TcpStream,
    pub login: LoginPacket,
    pub initial_position: PositionPacket,
    pub selected_hotbar_slot: i32,
    pub inventory_slots: Vec<PlayerInventorySlot>,
    pub health: HealthState,
}

impl PlayClient {
    pub async fn connect(host: &str, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::connect_with_block(host, name, Some(0)).await
    }

    pub async fn connect_with_block(
        host: &str,
        name: &str,
        expected_block: Option<i32>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let host = host.to_string();
        let name = name.to_string();
        super::retry_connect(|| {
            let host = host.clone();
            let name = name.clone();
            async move {
                let mut stream = TcpStream::connect(&host).await?;
                super::send_handshake(&mut stream, &host, NextState::Login).await?;
                let login = LoginStart::encode(&name, Uuid::from_u128(0));
                codec::write_packet(&mut stream, ids::login::START, &login).await?;
                let success =
                    super::expect(&mut stream, ids::login::SUCCESS, "login success").await?;
                validate_login_success(success.data, &name)?;
                codec::write_packet(&mut stream, ids::login::ACKNOWLEDGED, &[]).await?;
                complete_configuration(&mut stream).await?;
                let (login, selected_hotbar_slot, inventory_slots, health, initial_position) =
                    complete_play_bootstrap(&mut stream, expected_block).await?;
                Ok::<Self, Box<dyn std::error::Error>>(Self {
                    stream,
                    login,
                    initial_position,
                    selected_hotbar_slot,
                    inventory_slots,
                    health,
                })
            }
        })
        .await
    }
}
