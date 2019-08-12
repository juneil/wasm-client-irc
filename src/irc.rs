use lazy_static::lazy_static;
use regex::Regex;
use super::input;
use super::content;

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

pub struct IRC {
    input: input::Input,
    content: content::Content
}

impl IRC {
    pub fn new() -> IRC {
        IRC {
            input: input::Input::new(),
            content: content::Content::new()
        }
    }

    pub fn get_content(&self) -> content::Content {
        self.content.clone()
    }

    pub fn process_command(&self, raw: String) {
        if let Some(command) = Command::new(raw.as_ref()) {
            match command {
                Command { command: Some("ERROR"), .. } =>
                    self.content.insert_message(command.data.unwrap_or("").as_ref(), None),
                _ => self.content.insert_message("///", None)
            }
        }
    }
}