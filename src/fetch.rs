use crate::{auth::Token, block::Block};
use reqwest::{blocking::Client, StatusCode};

const SERVER_URL: &str = "http://localhost:8080";

type ActionHandlerResponse<T> = Result<(T, StatusCode), reqwest::Error>;

pub trait ResponseHandler<T> {
    fn handle_response(&self, success_msg: &str, fail_msg: &str) -> Option<&T>;
}

impl<T> ResponseHandler<T> for ActionHandlerResponse<T> {
    fn handle_response(&self, success_msg: &str, fail_msg: &str) -> Option<&T> {
        match &self {
            Ok(res) => {
                if res.1.is_success() {
                    println!("{}", success_msg);
                    return Some(&res.0);
                } else {
                    println!("{}", fail_msg);
                    return None;
                }
            }
            Err(_) => {
                println!("> Fehler");
                return None;
            }
        }
    }
}

pub struct ActionHandler {}

impl ActionHandler {
    pub fn new() -> ActionHandler {
        ActionHandler {}
    }

    pub fn start_block(&self, token: &Token) -> ActionHandlerResponse<()> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/block_start");
        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        Ok(((), res.status()))
    }

    pub fn start_pause(&self, token: &Token) -> ActionHandlerResponse<()> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/pause_start");
        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        Ok(((), res.status()))
    }

    pub fn end_block(&self, token: &Token) -> ActionHandlerResponse<()> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/block_end");
        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        Ok(((), res.status()))
    }

    pub fn end_pause(&self, token: &Token) -> ActionHandlerResponse<()> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/pause_end");
        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        Ok(((), res.status()))
    }

    pub fn get_current_block(&self, token: &Token) -> ActionHandlerResponse<Block> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/block_current");
        let res = client
            .get(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        let status = res.status();
        let block: Block = res.json()?;

        Ok((block, status))
    }

    pub fn get_all_blocks(&self, token: &Token) -> ActionHandlerResponse<Vec<Block>> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/block");
        let res = client
            .get(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        let status = res.status();
        let blocks: Vec<Block> = res.json()?;

        Ok((blocks, status))
    }
}
