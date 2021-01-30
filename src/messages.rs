use std::convert::TryFrom;

#[allow(dead_code)]
enum Message<'a> {
    Away {
        message: Option<&'a [u8]>,
    },
    Error {
        message: &'a [u8],
    },
    Info {
        server: Option<&'a [u8]>,
    },
    Join {
        channels: Vec<&'a [u8]>,
        keys: Option<Vec<&'a [u8]>>,
    },
    Kick {
        channel: &'a [u8],
        user: &'a [u8],
        comment: Option<&'a [u8]>,
    },
    Kill {
        nickname: &'a [u8],
        comment: &'a [u8],
    },
    List {
        channels: Option<Vec<&'a [u8]>>,
        server: &'a [u8],
    },
    Names {
        channels: Vec<&'a [u8]>,
    },
    Nick {
        nickname: &'a [u8],
    },
    Notice {
        nickname: &'a [u8],
        text: &'a [u8],
    },
    Part {
        channels: Vec<&'a [u8]>,
    },
    Pass {
        password: &'a [u8],
    },
    Ping {
        server1: &'a [u8],
        server2: Option<&'a [u8]>,
    },
    Pong {
        daemon: &'a [u8],
        daemon2: Option<&'a [u8]>,
    },
    Privmsg {
        receivers: Vec<&'a [u8]>,
        message: &'a [u8],
    },
    Quit {
        message: Option<&'a [u8]>,
    },
    Stats {
        query: Option<&'a [u8]>,
        server: Option<&'a [u8]>,
    },
    Time {
        server: Option<&'a [u8]>,
    },
    Topic {
        channel: &'a [u8],
        topic: &'a [u8],
    },
    User {
        username: &'a [u8],
        hostname: &'a [u8],
        servername: &'a [u8],
        realname: &'a [u8],
    },
    Users {
        server: Option<&'a [u8]>,
    },
    Version {
        server: Option<&'a [u8]>,
    },
    Whois {
        server: Option<&'a [u8]>,
        nickmask: Vec<&'a [u8]>,
    },
    Whowas {
        nickname: &'a [u8],
        count: Option<&'a [u8]>,
        server: Option<&'a [u8]>,
    },
}

fn split_one<'a>(a: &'a [u8], v: u8) -> (Option<&'a [u8]>, Option<&'a [u8]>) {
    let mut split_v = a.splitn(2, |&x| x == v);
    let left = split_v.next();
    let right = split_v.next();
    (left, right)
}

fn split_on_crlf<'a>(a: &'a [u8]) -> &'a [u8] {
    let mut split_v = a.splitn(2, |&x| x == '\n' as u8 || x == '\n' as u8);
    split_v.next().unwrap();
}

impl<'a> TryFrom<&'a [u8]> for Message<'a> {
    type Error = ();
    fn try_from(bytes: &'a [u8]) -> Result<Self, <Message<'a> as TryFrom<&'a [u8]>>::Error> {
        let bytes = split_on_crlf(bytes);
        let (command, right_side) = split_one(bytes, ' ' as u8);
        let command = command.unwrap();
        match command {
            b"AWAY" => {
                if let Some(r) = right_side {
                    let (_, message) = split_one(r, ':' as u8);
                    Ok(Message::Away { message })
                } else {
                    Ok(Message::Away { message: None })
                }
            }
            b"INFO" => Ok(Message::Info { server: None }),
            b"NICK" => {
                if let Some(r) = right_side {
                    let (n, right_side) = split_one(r, ' ' as u8);
                    if let Some(nickname) = n {
                        if !nickname.contains(&('' as u8)) {
                            return Ok(Message::Nick { nickname });
                        }
                    }
                }
                Err(())
            }
            b"KICK" => Err(()),
            b"KILL" => Err(()),
            b"VERSION" => Err(()),
            b"PASS" => {
                if let Some(password) = right_side {
                    Ok(Message::Pass { password })
                } else {
                    Err(())
                }
            }
            b"USER" => Err(()),
            b"USERS" => Err(()),
            b"QUIT" => Err(()),
            b"JOIN" => Err(()),
            b"PART" => Err(()),
            b"STATS" => Err(()),
            b"TOPIC" => Err(()),
            b"LIST" => Err(()),
            b"WHOIS" => Err(()),
            b"WHOWAS" => Err(()),
            b"PING" => Err(),
            b"PONG" => Err(()),
            b"ERROR" => Err(()),
            _ => Err(()),
        }
    }
}

#[test]
fn test_split_one() {
    assert_eq!((Some(b"foo".as_ref()), None), split_one(b"foo", ' ' as u8));
    assert_eq!(
        (Some(b"foo".as_ref()), Some(b"".as_ref())),
        split_one(b"foo ", ' ' as u8)
    );
    assert_eq!(
        (Some(b"foo".as_ref()), Some(b"bar".as_ref())),
        split_one(b"foo bar", ' ' as u8)
    );
}
