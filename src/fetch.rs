use crate::{auth::Token, block::Block};
use reqwest::blocking::Client;

pub struct Handler {}

const SERVER_URL: &str = "http://localhost:8080";

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }

    pub fn start_block(&self, token: &Token) -> Result<u16, reqwest::Error> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/block_start");
        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;
        let status = res.status();

        Ok(status.as_u16())
    }

    pub fn start_pause(&self, token: &Token) -> Result<u16, reqwest::Error> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/pause_start");
        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;
        let status = res.status();

        Ok(status.as_u16())
    }

    pub fn end_block(&self, token: &Token) -> Result<u16, reqwest::Error> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/block_end");
        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;
        let status = res.status();

        Ok(status.as_u16())
    }

    pub fn end_pause(&self, token: &Token) -> Result<u16, reqwest::Error> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/pause_end");
        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;
        let status = res.status();

        Ok(status.as_u16())
    }

    pub fn get_current_block(&self, token: &Token) -> Result<Block, reqwest::Error> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/block_current");
        let res = client
            .get(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        let block: Block = res.json()?;

        Ok(block)
    }

    pub fn get_all_blocks(&self, token: &Token) -> Result<Vec<Block>, reqwest::Error> {
        let client = Client::new();
        let url = format!("{SERVER_URL}/block");
        let res = client
            .get(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        let blocks: Vec<Block> = res.json()?;

        Ok(blocks)
    }
}
