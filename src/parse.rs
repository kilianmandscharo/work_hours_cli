pub enum Command {
    StartBlock,
    StartBlockHomeoffice,
    EndBlock,
    StartPause,
    EndPause,
    Current,
    All,
    Exit,
    Delete(i32),
    Unknown,
}

pub fn parse_command(command: &str) -> Command {
    match command {
        "start block" => Command::StartBlock,
        "start block homeoffice" => Command::StartBlockHomeoffice,
        "end block" => Command::EndBlock,
        "start pause" => Command::StartPause,
        "end pause" => Command::EndPause,
        "current" => Command::Current,
        "all" => Command::All,
        "exit" => Command::Exit,
        _ => {
            let elements: Vec<&str> = command.split(" ").collect();
            match elements[0] {
                "delete" => {
                    let id = elements[1].trim().parse::<i32>();
                    match id {
                        Ok(id) => Command::Delete(id),
                        Err(_) => Command::Unknown,
                    }
                }
                _ => Command::Unknown,
            }
        }
    }
}
