use crate::protocol::configuration::{self, KnownPack};
use crate::protocol::ids;
use crate::session::SessionState;
use crate::session::error::ConnectionError;
use crate::session::io::{codec_error, protocol_error, read_until_packet, write_packet};
use tokio::net::TcpStream;

pub async fn handle_configuration(stream: &mut TcpStream) -> Result<(), ConnectionError> {
    let phase = SessionState::Configuration;
    let known_packs = configuration::encode_select_known_packs();
    write_packet(stream, phase, ids::config::SELECT_KNOWN_PACKS, &known_packs).await?;
    let selected = read_until_packet(
        stream,
        phase,
        ids::config::SERVERBOUND_SELECT_KNOWN_PACKS,
        ignored_client_packets(),
    )
    .await?;
    let packs = configuration::decode_known_packs(selected.data)
        .map_err(|error| codec_error(phase, error))?;
    if !packs.iter().any(is_vanilla_core_pack) {
        return Err(protocol_error(
            phase,
            "client did not select vanilla core pack",
        ));
    }
    for registry in configuration::encode_registry_data() {
        write_packet(stream, phase, ids::config::REGISTRY_DATA, &registry).await?;
    }
    write_packet(
        stream,
        phase,
        ids::config::TAGS,
        &configuration::encode_tags(),
    )
    .await?;
    let features = configuration::encode_enabled_features();
    write_packet(stream, phase, ids::config::FEATURE_FLAGS, &features).await?;
    write_packet(stream, phase, ids::config::FINISH, &[]).await?;
    read_until_packet(stream, phase, ids::config::FINISH, ignored_client_packets()).await?;
    Ok(())
}

fn ignored_client_packets() -> &'static [i32] {
    &[
        ids::config::SERVERBOUND_SETTINGS,
        ids::config::SERVERBOUND_CUSTOM_PAYLOAD,
    ]
}

fn is_vanilla_core_pack(pack: &KnownPack) -> bool {
    let vanilla = KnownPack::vanilla_core();
    pack == &vanilla
}
