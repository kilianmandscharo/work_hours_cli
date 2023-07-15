pub enum Command {
    BlockStart(bool),
    BlockEnd,
    BlockDelete(i32),
    BlockCurrent,
    BlockAll,
    BlockUpdateStart((i32, String)),
    BlockUpdateEnd((i32, String)),
    BlockUpdateHomeoffice((i32, bool)),
    PauseStart,
    PauseEnd,
    PauseDelete(i32),
    PauseUpdateStart((i32, String)),
    PauseUpdateEnd((i32, String)),
    Exit,
    Unknown,
}

pub fn parse_command(command: &str) -> Command {
    let split: Vec<&str> = command.split(" ").collect();
    match split[0] {
        "block" => parse_block_command(&split),
        "pause" => parse_pause_command(&split),
        "exit" => Command::Exit,
        _ => Command::Unknown,
    }
}

fn parse_block_command(split: &Vec<&str>) -> Command {
    let len = split.len();

    if len == 1 {
        return Command::Unknown;
    }

    match split[1] {
        "start" => {
            if len < 3 {
                return Command::BlockStart(false);
            }
            let homeoffice = split[2].trim().parse::<bool>();
            match homeoffice {
                Ok(homeoffice) => Command::BlockStart(homeoffice),
                Err(_) => Command::BlockStart(false),
            }
        }
        "end" => Command::BlockEnd,
        "delete" => {
            if len < 3 {
                return Command::Unknown;
            }
            let id = split[2].trim().parse::<i32>();
            match id {
                Ok(id) => Command::BlockDelete(id),
                Err(_) => Command::Unknown,
            }
        }
        "current" => Command::BlockCurrent,
        "all" => Command::BlockAll,
        "update" => parse_block_update_command(split),
        _ => Command::Unknown,
    }
}

fn parse_block_update_command(split: &Vec<&str>) -> Command {
    if split.len() < 5 {
        return Command::Unknown;
    }

    let id = split[2].trim().parse::<i32>();
    match id {
        Ok(id) => match split[3] {
            "start" => Command::BlockUpdateStart((id, split[4].to_string())),
            "end" => Command::BlockUpdateEnd((id, split[4].to_string())),
            "homeoffice" => {
                let homeoffice = split[4].trim().parse::<bool>();
                match homeoffice {
                    Ok(homeoffice) => Command::BlockUpdateHomeoffice((id, homeoffice)),
                    Err(_) => Command::Unknown,
                }
            }
            _ => Command::Unknown,
        },
        Err(_) => Command::Unknown,
    }
}

fn parse_pause_command(split: &Vec<&str>) -> Command {
    let len = split.len();

    match split[1] {
        "start" => Command::PauseStart,
        "end" => Command::PauseEnd,
        "delete" => {
            if len < 3 {
                return Command::Unknown;
            }
            let id = split[2].trim().parse::<i32>();
            match id {
                Ok(id) => Command::PauseDelete(id),
                Err(_) => Command::Unknown,
            }
        }
        "update" => parse_pause_update_command(split),
        _ => Command::Unknown,
    }
}

fn parse_pause_update_command(split: &Vec<&str>) -> Command {
    if split.len() < 5 {
        return Command::Unknown;
    }

    let id = split[2].trim().parse::<i32>();
    match id {
        Ok(id) => match split[3] {
            "start" => Command::PauseUpdateStart((id, split[4].to_string())),
            "end" => Command::PauseUpdateEnd((id, split[4].to_string())),
            _ => Command::Unknown,
        },
        Err(_) => Command::Unknown,
    }
}
