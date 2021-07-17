use futures::stream::StreamExt;
use irc_proto::command::Command;
use irc_proto::irc::IrcCodec;
use irc_proto::message::Message;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Debug, Default, Clone)]
pub struct Server {
    state: Arc<ServerData>,
}

#[derive(Debug, Default)]
struct ServerData {
    rooms: RwLock<HashMap<String, Vec<String>>>,
    users: RwLock<HashMap<String, mpsc::Sender<Message>>>,
}

impl Server {
    pub async fn handle_conn(&self, conn: TcpStream) {
        let mut u = UserSession::from(conn);

        // User connection/setup process
        while let Some(Ok(m)) = u.tcp_reader.next().await {
            println!("{:?}", m);
            match m.command {
                Command::PASS(password) => u.server_pass = Some(password),
                Command::NICK(nick) => u.nick = Some(nick),
                Command::USER(username, hostname, realname) => {
                    u.user_info = Some(UserInfo {
                        username,
                        hostname,
                        realname,
                    });
                    break;
                }
                _ => {}
            }
        }
    }
}

struct UserSession {
    tcp_reader: futures::stream::SplitStream<Framed<TcpStream, IrcCodec>>,
    tcp_writer: futures::stream::SplitSink<Framed<TcpStream, IrcCodec>, Message>,
    server_pass: Option<String>,
    nick: Option<String>,
    user_info: Option<UserInfo>,
}

#[derive(Debug, Default)]
struct UserInfo {
    username: String,
    hostname: String,
    realname: String,
}

impl From<TcpStream> for UserSession {
    fn from(stream: TcpStream) -> Self {
        let framed_tcp = Framed::new(stream, IrcCodec::new("utf-8").unwrap());
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
