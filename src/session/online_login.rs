use crate::protocol::encryption::EncryptedStream;
use crate::protocol::ids;
use crate::protocol::{codec, login};
use crate::session::SessionState;
use crate::session::auth;
use crate::session::error::ConnectionError;
use crate::session::handler::ServerContext;
use crate::session::io::{codec_error, expect_packet, protocol_error, write_packet};
use rand::RngCore;
use rsa::pkcs8::EncodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use tokio::io::AsyncWrite;
use tokio::net::TcpStream;
use uuid::Uuid;

pub struct AuthenticatedStream {
    pub stream: EncryptedStream<TcpStream>,
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Debug)]
pub struct LoginKey {
    private_key: RsaPrivateKey,
    public_der: Vec<u8>,
}

impl LoginKey {
    pub fn generate() -> Result<Self, String> {
        let mut rng = rand::rngs::OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 1024).map_err(|error| error.to_string())?;
        let public_der = private_key
            .to_public_key()
            .to_public_key_der()
            .map_err(|error| error.to_string())?;
        Ok(Self {
            private_key,
            public_der: public_der.as_ref().to_vec(),
        })
    }
}

pub async fn authenticate(
    mut stream: TcpStream,
    context: &ServerContext,
    name: &str,
) -> Result<AuthenticatedStream, ConnectionError> {
    let phase = SessionState::Login;
    let mut rng = rand::rngs::OsRng;
    let mut verify_token = [0_u8; 16];
    rng.fill_bytes(&mut verify_token);
    let request = login::encode_encryption_request(&context.login_key.public_der, &verify_token);
    write_packet(&mut stream, phase, ids::login::ENCRYPTION_REQUEST, &request).await?;

    let response = expect_packet(&mut stream, phase, ids::login::ENCRYPTION_RESPONSE).await?;
    let response = login::decode_encryption_response(response.data)
        .map_err(|error| codec_error(phase, error))?;
    let secret = decrypt_secret(&context.login_key.private_key, &response.shared_secret)?;
    let token = context
        .login_key
        .private_key
        .decrypt(Pkcs1v15Encrypt, &response.verify_token)
        .map_err(|_| protocol_error(phase, "verify token decrypt failed"))?;
    if token != verify_token {
        send_auth_failed(&mut stream).await?;
        return Err(protocol_error(phase, "verify token mismatch"));
    }
    let hash = auth::server_hash(&secret, &context.login_key.public_der);
    match auth::verify_has_joined(&context.config.session_server_url, name, &hash).await {
        Ok(profile) => Ok(AuthenticatedStream {
            stream: EncryptedStream::new(stream, &secret),
            uuid: profile.uuid,
            name: profile.name,
        }),
        Err(_) => {
            let mut encrypted = EncryptedStream::new(stream, &secret);
            send_auth_failed(&mut encrypted).await?;
            Err(protocol_error(phase, "session verification failed"))
        }
    }
}

fn decrypt_secret(
    private_key: &RsaPrivateKey,
    encrypted: &[u8],
) -> Result<[u8; 16], ConnectionError> {
    private_key
        .decrypt(Pkcs1v15Encrypt, encrypted)
        .map_err(|_| protocol_error(SessionState::Login, "shared secret decrypt failed"))?
        .try_into()
        .map_err(|_| protocol_error(SessionState::Login, "invalid shared secret length"))
}

async fn send_auth_failed<W>(stream: &mut W) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    let json = serde_json::json!({ "text": "Authentication failed" }).to_string();
    let mut payload = Vec::new();
    codec::write_string(&mut payload, &json);
    write_packet(
        stream,
        SessionState::Login,
        ids::login::DISCONNECT,
        &payload,
    )
    .await
}
