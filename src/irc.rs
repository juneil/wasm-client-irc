use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?m)^(?::(([^@!\s]*)(?:(?:!([^@]*))?@([^\s]*))?)\s)?([^\s]+)((?:\s[^:\s][^\s]*){0,14})(?:\s:?(.*))?$").unwrap();
}

#[derive(Debug, Clone)]
pub struct Command {
    prefix: Option<String>,
    nickname: Option<String>,
    username: Option<String>,
    hostname: Option<String>,
    command: Option<String>,
    params: Option<String>,
    data: Option<String>
}

impl Command {
    pub fn new(raw: String) -> Option<Command> {
        match RE.captures(raw.as_ref()) {
            Some(caps) => Some(Command {
                prefix: caps.get(1).map(|s| String::from(s.as_str())),
                nickname: caps.get(2).map(|s| String::from(s.as_str())),
                username: caps.get(3).map(|s| String::from(s.as_str())),
                hostname: caps.get(4).map(|s| String::from(s.as_str())),
                command: caps.get(5).map(|s| String::from(s.as_str())),
                params: caps.get(6).map(|s| String::from(s.as_str())),
                data: caps.get(7).map(|s| String::from(s.as_str())),
            }),
            None => None
        }
    }
}