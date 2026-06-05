pub enum Command {
    Ping,
    Set { key: String, value: String },
    Get { key: String },
}

pub fn parse(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["PING"] => Some(Command::Ping),
        ["SET", key, value] => Some(Command::Set {
            key: key.to_string(),
            value: value.to_string(),
        }),
        ["GET", key] => Some(Command::Get {
            key: key.to_string(),
        }),
        _ => None,
    }
}
