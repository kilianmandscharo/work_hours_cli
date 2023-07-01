// use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
// use std::path::Path;

mod auth;

#[derive(Serialize, Deserialize)]
struct BlockCreate {
    start: String,
    end: String,
    pauses: Vec<PauseCreate>,
}

#[derive(Serialize, Deserialize)]
struct PauseCreate {
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

// const SERVER_URL: &str = "http://localhost:8080";

fn main() {
    // loop {
    //     println!("Enter a command:");
    //
    //     let mut command = String::new();
    //
    //     stdin()
    //         .read_line(&mut command)
    //         .expect("Could not read line");
    //
    //     match command.trim() {
    //         "get" => get_blocks(),
    //         "add" => add_block(),
    //         "exit" => break,
    //         _ => continue,
    //     }
    // }
    // add_block().expect("could not parse struct to json");
    // let token = login().unwrap();
    // add_block(token).unwrap();
}

// fn get_blocks() {
//     println!("selected get");
// }
//

// fn now_with_offset(offset: i64) -> String {
//     let with_offset = Utc::now() - Duration::hours(offset);
//     with_offset.to_rfc3339()
// }
//
// fn test_block_json() -> serde_json::Result<String> {
//     let block_start = now_with_offset(10);
//     let block_end = now_with_offset(1);
//     let pause_start = now_with_offset(6);
//     let pause_end = now_with_offset(5);
//
//     let pause = PauseCreate {
//         start: pause_start,
//         end: pause_end,
//     };
//
//     let block = BlockCreate {
//         start: block_start,
//         end: block_end,
//         pauses: vec![pause],
//     };
//
//     let block = serde_json::to_string(&block)?;
//
//     Ok(block)
// }
//
// fn add_block(token: String) -> Result<(), ()> {
//     let block = test_block_json().unwrap();
//
//     let client = reqwest::blocking::Client::new();
//
//     let path = Path::new(SERVER_URL).join("block");
//     let path = path.to_str().unwrap();
//
//     let res = client
//         .post(path)
//         .header("Authorization", format!("Bearer {}", token))
//         .body(block)
//         .send()
//         .unwrap();
//
//     println!("{}", res.status());
//
//     Ok(())
// }

// struct Block {
//     id: i32,
//     start: String,
//     end: String,
//     pauses: Vec<Pause>,
// }
//
// struct Pause {
//     id: i32,
//     start: String,
//     end: String,
// }
