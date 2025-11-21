use std::{net::SocketAddr, sync::{Arc, Mutex}, collections::HashMap};
use futures_util::{StreamExt, SinkExt, TryStreamExt};
use log::{info, error};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let addr = "127.0.0.1:8080";
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        let state = state.clone();
        tokio::spawn(accept_connection(state, stream, addr));
    }

    Ok(())
}

async fn accept_connection(peer_map: PeerMap, stream: TcpStream, addr: SocketAddr) {
    info!("Incoming TCP connection from: {}", addr);

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake occurred: {}", e);
            return;
        }
    };

    info!("WebSocket connection established: {}", addr);

    let (tx, mut rx) = unbounded_channel();
    peer_map.lock().unwrap().insert(addr, tx);

    let (mut outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        // We want to broadcast the message to everyone else.
        if msg.is_close() {
             return futures_util::future::ok(());
        }

        let peers = peer_map.lock().unwrap();
        let broadcast_recipients = peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            let _ = recp.send(msg.clone());
        }

        futures_util::future::ok(())
    });

    let receive_from_others = async {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = outgoing.send(msg).await {
                error!("Error sending message: {}", e);
                break;
            }
        }
    };

    tokio::select! {
        val = broadcast_incoming => {
            if let Err(e) = val {
                 error!("Error processing incoming messages: {}", e);
            }
        }
        _ = receive_from_others => {
             // receive_from_others finishes when rx is closed or outgoing fails
        }
    }

    info!("{} disconnected", addr);
    peer_map.lock().unwrap().remove(&addr);
}
