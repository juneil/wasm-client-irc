use lazy_static::lazy_static;
use regex::Regex;
use super::input;
use super::content;
use super::log;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?m)^(?::(([^@!\s]*)(?:(?:!([^@]*))?@([^\s]*))?)\s)?([^\s]+)((?:\s[^:\s][^\s]*){0,14})(?:\s:?(.*))?$").unwrap();
}

#[derive(Debug, Clone)]
pub struct Command<'t> {
    prefix: Option<&'t str>,
    nickname: Option<&'t str>,
    username: Option<&'t str>,
    hostname: Option<&'t str>,
    command: Option<&'t str>,
    params: Option<&'t str>,
    data: Option<&'t str>
}

impl <'t> Command<'t> {
    pub fn new(raw: &'t str) -> Option<Command<'t>> {
        match RE.captures(raw.as_ref()) {
            Some(caps) => Some(Command {
                prefix: caps.get(1).map(|s| s.as_str()),
                nickname: caps.get(2).map(|s| s.as_str()),
                username: caps.get(3).map(|s| s.as_str()),
                hostname: caps.get(4).map(|s| s.as_str()),
                command: caps.get(5).map(|s| s.as_str()),
                params: caps.get(6).map(|s| s.as_str()),
                data: caps.get(7).map(|s| s.as_str()),
            }),
            None => None
        }
    }
}

pub enum MessageStyle {
    Normal,
    Error
}

pub struct IRC<F> {
    input: input::Input,
    content: content::Content,
    connected: bool,
    nickname: String,
    handler: F
}

impl<F> IRC<F> where F: FnMut(String) -> () {
    pub fn new(handler: F) -> IRC<F> {
        IRC {
            input: input::Input::new(),
            content: content::Content::new(),
            connected: false,
            nickname: String::from("Juneil"),
            handler: handler
        }
    }

    pub fn send(&mut self, data: String) {
        log!("send: {}", data);
        (self.handler)(data);
    }

    pub fn register(&mut self) {
        self.send(format!("USER {} * * :{}", self.nickname, self.nickname));
        self.send(format!("NICK {}", self.nickname));
        self.connected = true;
    }

    pub fn process_command(&mut self, raw: String) {
        if self.connected == false {
            self.register();
        }
        if let Some(command) = Command::new(raw.as_ref()) {
            match command {
                Command { command: Some("ERROR"), .. } => {
                    self.connected = false;
                    self.content.insert_message(command.data.unwrap_or("").as_ref(), None, Some(MessageStyle::Error))
                },
                Command { command: Some("PING"), .. } => self.send(format!("PONG :{}", command.data.unwrap_or(""))),
                _ => ()
            }
        }
    }
}