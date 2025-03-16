use futures_util::{StreamExt, SinkExt};
use reqwest::{
    Client,
    Response,
    header::{HeaderMap, HeaderValue}
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        self,
        Utf8Bytes, 
        protocol::{
            Message, 
            CloseFrame, 
            frame::coding::CloseCode
        }
    },
};

#[allow(dead_code)]
#[derive(serde::Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized
}

/// ### Simple Example of HTTP RPC request
pub async fn get_transaction<U, S>(url: U, signature: S, commitment: CommitmentLevel) -> Result<(), Box<dyn std::error::Error>> 
where
    U: ToString + reqwest::IntoUrl,
    S: AsRef<[u8]> + serde::Serialize
{
    if commitment == CommitmentLevel::Processed { return Err("Commitment::Processed is not supported for getTransaction method!".into()); }

    // building http client
    let mut headers: HeaderMap = HeaderMap::with_capacity(1); 
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client: Client = Client::builder().default_headers(headers).build()?;

    let request_json_rpc: serde_json::Value = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": [
            signature,
            {
                "encoding": "json",
                "commitment": commitment,
                "maxSupportedTransactionVersion": 0
            }
        ]
    });

    let res: Response = client.post(url).json(&request_json_rpc).send().await?;
    let res_body: String = res.text().await?;

    // using serde_json::Value for simplicity, however it's recommended to use getTransaction scheme instead
    if let Ok(tx) = serde_json::from_str::<serde_json::Value>(&res_body) {
        log::info!("{:#?}", tx);
    }

    Ok(())  // for simplicity sake returns nothing
}

/// ### Simple WS RPC Stream Example without reconnection logic, however with proper stream cancelation
pub async fn account_subscribe<U, P>(url: U, pubkey: P, commitment: CommitmentLevel) -> Result<(), Box<dyn std::error::Error>> 
where
    U: ToString + tungstenite::client::IntoClientRequest + Unpin,
    P: AsRef<[u8]> + serde::Serialize
{
    let (ws_stream, _) = connect_async(url)
        .await
        .map_err(|_| "Failed to make a handshake!")?;

    let (mut write, mut read) = ws_stream.split();

    let request_json_rpc: serde_json::Value = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "accountSubscribe",
        "params": [
            pubkey,
            {
                "encoding": "jsonParsed",
                "commitment": commitment
            }
        ]
    });

    if let Err(e) = write.send(Message::text(request_json_rpc.to_string())).await {
        if matches!(e, tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) {
            log::error!("Server closed connection after the handshake, but before sending a subscription request!");
        } else {
            log::trace!("An error occurred while sending subscription request!");
            try_to_close_connection(&mut write, None).await;
        }
        return Err(e.into());
    }

    loop {
        tokio::select! {
            Some(msg) = read.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        log::info!("{:#?}", text);
                        // process text ... 
                    },
                    Ok(Message::Ping(v)) => {
                        if let Err(_) = write.send(Message::Pong(v)).await {
                            log::error!("Failed to send Pong Frame!");
                            continue;
                        }
                        log::error!("Sent Pong Frame!");
                    },
                    Ok(Message::Close(frame)) => {
                        log::error!("Received Close Frame! Closing stream.");
                        try_to_close_connection(&mut write, frame).await;
                    },
                    Ok(_) => {},
                    Err(e) => {
                        match e {
                            tungstenite::Error::ConnectionClosed => break log::info!("Connection is properly closed!"),
                            _ => return Err(e.into())
                        }
                    }
                }
            },

            _ = tokio::time::sleep(tokio::time::Duration::from_millis(10000)) => {
                if let Err(e) = write.send(Message::Ping(vec![].into())).await {
                    log::error!("Failed to send a Heartbeat Ping! {e}");
                }
            }
        }
    }

    Ok(())
}

async fn try_to_close_connection<T: SinkExt<Message> + Unpin>(write: &mut T, close_frame: Option<CloseFrame>) -> () {
    if let Err(_) = write.send(Message::Close(close_frame)).await {
        log::error!("Failed to properly close connection!");
    }
}