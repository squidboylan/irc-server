use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct Server {
    state: Arc<ServerData>,
}

#[derive(Debug)]
struct ServerData {
    rooms: RwLock<HashMap<String, Vec<String>>>,
    users: RwLock<HashMap<String, mpsc::Sender<Vec<u8>>>>,
}

/*
impl Server {
    pub async fn handle_conn(&self, conn: TcpStream) {

        let u = UserSession::from(conn);

        let buf = [0;512];
        while Ok(_) = u.tcp_reader.read(&mut buf).await {

        }

    }
}
*/

#[derive(Debug)]
struct UserSession {
    tcp_reader: OwnedReadHalf,
    tcp_writer: OwnedWriteHalf,
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
        let (tcp_reader, tcp_writer) = stream.into_split();
        UserSession {
            tcp_reader,
            tcp_writer,
            server_pass: None,
            nick: None,
            user_info: None,
        }
    }
}
