use std::collections::HashMap;
use std::sync::Arc;
use std::convert::TryFrom;
use futures::stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use crate::messages::Message;

#[derive(Debug, Default, Clone)]
pub struct Server {
    state: Arc<ServerData>,
}

#[derive(Debug, Default)]
struct ServerData {
    rooms: RwLock<HashMap<String, Vec<String>>>,
    users: RwLock<HashMap<String, mpsc::Sender<Vec<u8>>>>,
}

impl Server {
    pub async fn handle_conn(&self, conn: TcpStream) {

        let mut u = UserSession::from(conn);

        while let Some(Ok(line)) = u.tcp_reader.next().await {
            let m = Message::try_from(line.as_str());
            println!("{:?}", m);
        }
    }
}

#[derive(Debug)]
struct UserSession {
    tcp_reader: futures::stream::SplitStream<Framed<TcpStream, LinesCodec>>,
    tcp_writer: futures::stream::SplitSink<Framed<TcpStream, LinesCodec>, String>,
    server_pass: Option<String>,
    nick: Option<String>,
    user_info: Option<UserInfo>,
}

#[derive(Debug, Default)]
struct UserInfo {
    username: String,
    hostname: String,
    servername: String,
    realname: String,
}

impl From<TcpStream> for UserSession {
    fn from(stream: TcpStream) -> Self {
        let framed_tcp = Framed::new(stream, LinesCodec::new_with_max_length(512));
        let (tcp_writer, tcp_reader) = framed_tcp.split();
        UserSession {
            tcp_reader,
            tcp_writer,
            server_pass: None,
            nick: None,
            user_info: None,
        }
    }
}
