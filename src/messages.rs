use regex::Regex;
use std::convert::TryFrom;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Message<'a> {
    Away {
        message: Option<&'a str>,
    },
    Error {
        message: &'a str,
    },
    Info {
        server: Option<&'a str>,
    },
    Join {
        channels: Vec<&'a str>,
        keys: Option<Vec<&'a str>>,
    },
    Kick {
        channel: &'a str,
        user: &'a str,
        comment: Option<&'a str>,
    },
    Kill {
        nickname: &'a str,
        comment: &'a str,
    },
    List {
        channels: Option<Vec<&'a str>>,
        server: &'a str,
    },
    Names {
        channels: Vec<&'a str>,
    },
    Nick {
        nickname: &'a str,
    },
    Notice {
        nickname: &'a str,
        text: &'a str,
    },
    Part {
        channels: Vec<&'a str>,
    },
    Pass {
        password: &'a str,
    },
    Ping {
        server1: &'a str,
        server2: Option<&'a str>,
    },
    Pong {
        daemon: &'a str,
        daemon2: Option<&'a str>,
    },
    Privmsg {
        receivers: Vec<&'a str>,
        message: &'a str,
    },
    Quit {
        message: Option<&'a str>,
    },
    Stats {
        query: Option<&'a str>,
        server: Option<&'a str>,
    },
    Time {
        server: Option<&'a str>,
    },
    Topic {
        channel: &'a str,
        topic: &'a str,
    },
    User {
        username: &'a str,
        hostname: &'a str,
        servername: &'a str,
        realname: &'a str,
    },
    Users {
        server: Option<&'a str>,
    },
    Version {
        server: Option<&'a str>,
    },
    Whois {
        server: Option<&'a str>,
        nickmask: Vec<&'a str>,
    },
    Whowas {
        nickname: &'a str,
        count: Option<&'a str>,
        server: Option<&'a str>,
    },
}

impl<'a> TryFrom<&'a str> for Message<'a> {
    type Error = ();
    fn try_from(bytes: &'a str) -> Result<Self, <Message<'a> as TryFrom<&'a str>>::Error> {
        let re = Regex::new(r"^AWAY :(.*)?\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Away {
                message: caps.get(1).map(|x| x.as_str()),
            });
        }
        let re = Regex::new(r"^NICK ([[:alpha:]0-9])\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Nick { nickname: caps.get(1).unwrap().as_str() });
        }
        let re = Regex::new(r"^INFO (.*)?\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Info {
                server: caps.get(1).map(|x| x.as_str()),
            });
        }
        let re = Regex::new(r"^PASS (.*)\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Pass { password: caps.get(1).unwrap().as_str() });
        }
        let re = Regex::new(r"^KICK (.*) (.*) (.*)?\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Kick {
                channel: caps.get(1).unwrap().as_str(),
                user: caps.get(2).unwrap().as_str(),
                comment: caps.get(3).map(|x| x.as_str()),
            });
        }
        let re = Regex::new(r"^VERSION (.*)?\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Version {
                server: caps.get(1).map(|x| x.as_str()),
            });
        }
        let re = Regex::new(r"^KILL (.*) (.*)\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Kill {
                nickname: caps.get(1).unwrap().as_str(),
                comment: caps.get(2).unwrap().as_str(),
            });
        }
        let re = Regex::new(r"^PING (.*) (.*)\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Ping {
                server1: caps.get(1).unwrap().as_str(),
                server2: caps.get(2).map(|x| x.as_str()),
            });
        }
        let re = Regex::new(r"^PONG (.*) (.*)\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Pong {
                daemon1: caps.get(1).unwrap().as_str(),
                daemon2: caps.get(2).map(|x| x.as_str()),
            });
        }
        let re = Regex::new(r"^ERROR :(.*)\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Error {
                message: caps.get(1).unwrap().as_str(),
            });
        }
        let re = Regex::new(r"^QUIT :(.*)?\r?\n?$").unwrap();
        if let Some(caps) = re.captures(bytes) {
            return Ok(Message::Quit {
                message: caps.get(1).map(|x| x.as_str()),
            });
        }
        match bytes {
            "USER" => Err(()),
            "USERS" => Err(()),
            "JOIN" => Err(()),
            "PART" => Err(()),
            "STATS" => Err(()),
            "TOPIC" => Err(()),
            "LIST" => Err(()),
            "WHOIS" => Err(()),
            "WHOWAS" => Err(()),
            _ => Err(()),
        }
    }
}
