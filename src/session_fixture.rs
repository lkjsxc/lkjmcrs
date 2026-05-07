use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const FIXTURE_UUID: &str = "00112233445566778899aabbccddeeff";

pub async fn serve(bind: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(bind).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(error) = handle(stream).await {
                tracing::debug!(error = %error, "session fixture request failed");
            }
        });
    }
}

async fn handle(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0_u8; 2048];
    let read = stream.read(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..read]);
    let first_line = request.lines().next().unwrap_or_default();
    let body = if first_line.starts_with("GET /session/minecraft/hasJoined?")
        && first_line.contains("username=OnlineProbe")
        && first_line.contains("serverId=")
    {
        format!(r#"{{"id":"{FIXTURE_UUID}","name":"OnlineProbe"}}"#)
    } else {
        String::new()
    };
    let status = if body.is_empty() {
        "HTTP/1.1 204 No Content"
    } else {
        "HTTP/1.1 200 OK"
    };
    let response = format!(
        "{status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
