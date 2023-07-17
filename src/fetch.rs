use crate::{
    auth::Token,
    block::Block,
    error::FetchError,
    prompt::{error_text, success_text},
};
use reqwest::{blocking::Client, StatusCode};
use serde::Serialize;

const SERVER_URL: &str = "http://localhost:8080";

#[derive(Serialize)]
struct BodyStart<'a> {
    start: &'a str,
}

#[derive(Serialize)]
struct BodyEnd<'a> {
    end: &'a str,
}

#[derive(Serialize)]
struct BodyHomeoffice {
    homeoffice: bool,
}

type ActionHandlerResponse<T> = Result<(T, StatusCode), FetchError>;

pub trait ResponseHandler<T> {
    fn handle_response(self, success_msg: &str, fail_msg: &str) -> Option<T>;
}

impl<T> ResponseHandler<T> for ActionHandlerResponse<T> {
    fn handle_response(self, success_msg: &str, fail_msg: &str) -> Option<T> {
        match self {
            Ok(res) => {
                if res.1.is_success() {
                    println!("{}", success_text(success_msg));
                    return Some(res.0);
                } else {
                    println!("{}", error_text(fail_msg));
                    return None;
                }
            }
            Err(err) => {
                match err {
                    FetchError::JSONError(_) => println!("{}", error_text(fail_msg)),
                    FetchError::HTTPError(_) => {
                        println!("{}", error_text("> Netzwerk Fehler"))
                    }
                };
                return None;
            }
        }
    }
}

pub struct ActionHandler {
    blocks: Option<Vec<Block>>,
    current_block: Option<Block>,
}

impl ActionHandler {
    pub fn new() -> ActionHandler {
        ActionHandler {
            blocks: None,
            current_block: None,
        }
    }

    fn clear_cache(&mut self) {
        self.blocks = None;
        self.current_block = None;
    }

    fn toggle_current_item(&mut self, route: &str, token: &Token) -> ActionHandlerResponse<()> {
        self.clear_cache();

        let client = Client::new();
        let url = format!("{SERVER_URL}/{route}");
        let res = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        Ok(((), res.status()))
    }

    pub fn start_block(&mut self, token: &Token, homeoffice: bool) -> ActionHandlerResponse<()> {
        self.toggle_current_item(
            &format!("current_block_start?homeoffice={homeoffice}"),
            token,
        )
    }

    pub fn end_block(&mut self, token: &Token) -> ActionHandlerResponse<()> {
        self.toggle_current_item("current_block_end", token)
    }

    pub fn start_pause(&mut self, token: &Token) -> ActionHandlerResponse<()> {
        self.toggle_current_item("current_pause_start", token)
    }

    pub fn end_pause(&mut self, token: &Token) -> ActionHandlerResponse<()> {
        self.toggle_current_item("current_pause_end", token)
    }

    pub fn get_current_block(&mut self, token: &Token) -> ActionHandlerResponse<Block> {
        if let Some(ref block) = self.current_block {
            return Ok((block.clone(), StatusCode::OK));
        }

        let client = Client::new();
        let url = format!("{SERVER_URL}/block_current");
        let res = client
            .get(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        let status = res.status();
        let text = res.text()?;
        let block: Block = serde_json::from_str(&text)?;

        self.current_block = Some(block.clone());

        Ok((block, status))
    }

    pub fn get_all_blocks(&mut self, token: &Token) -> ActionHandlerResponse<Vec<Block>> {
        if let Some(ref blocks) = self.blocks {
            return Ok((blocks.clone(), StatusCode::OK));
        }

        let client = Client::new();
        let url = format!("{SERVER_URL}/block");
        let res = client
            .get(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        let status = res.status();
        let text = res.text()?;
        let blocks: Vec<Block> = serde_json::from_str(&text)?;

        self.blocks = Some(blocks.clone());

        Ok((blocks, status))
    }

    fn delete_item(&mut self, route: &str, id: i32, token: &Token) -> ActionHandlerResponse<()> {
        self.clear_cache();

        let client = Client::new();
        let url = format!("{SERVER_URL}/{route}/{id}");
        let res = client
            .delete(url)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        Ok(((), res.status()))
    }

    pub fn delete_block(&mut self, id: i32, token: &Token) -> ActionHandlerResponse<()> {
        self.delete_item("block", id, token)
    }

    pub fn delete_pause(&mut self, id: i32, token: &Token) -> ActionHandlerResponse<()> {
        self.delete_item("pause", id, token)
    }

    // fn get_block_by_id(&self, id: i32, token: &Token) -> ActionHandlerResponse<Block> {
    //     let client = Client::new();
    //     let url = format!("{SERVER_URL}/block/{id}");
    //     let res = client
    //         .get(url)
    //         .header("Authorization", format!("Bearer {}", token.token_string()))
    //         .send()?;
    //
    //     let status = res.status();
    //     let text = res.text()?;
    //     let block: Block = serde_json::from_str(&text)?;
    //
    //     Ok((block, status))
    // }

    fn update_item<T>(
        &mut self,
        route: &str,
        id: i32,
        body: T,
        token: &Token,
    ) -> ActionHandlerResponse<()>
    where
        T: Serialize,
    {
        self.clear_cache();

        let body = serde_json::to_string(&body)?;

        let client = Client::new();
        let url = format!("{SERVER_URL}/{route}/{id}");
        let res = client
            .put(url)
            .body(body)
            .header("Authorization", format!("Bearer {}", token.token_string()))
            .send()?;

        Ok(((), res.status()))
    }

    pub fn update_block_start(
        &mut self,
        id: i32,
        start: &str,
        token: &Token,
    ) -> ActionHandlerResponse<()> {
        self.update_item("block_start", id, BodyStart { start }, token)
    }

    pub fn update_block_end(
        &mut self,
        id: i32,
        end: &str,
        token: &Token,
    ) -> ActionHandlerResponse<()> {
        self.update_item("block_end", id, BodyEnd { end }, token)
    }

    pub fn update_block_homeoffice(
        &mut self,
        id: i32,
        homeoffice: bool,
        token: &Token,
    ) -> ActionHandlerResponse<()> {
        self.update_item("block_homeoffice", id, BodyHomeoffice { homeoffice }, token)
    }

    pub fn update_pause_start(
        &mut self,
        id: i32,
        start: &str,
        token: &Token,
    ) -> ActionHandlerResponse<()> {
        self.update_item("pause_start", id, BodyStart { start }, token)
    }

    pub fn update_pause_end(
        &mut self,
        id: i32,
        end: &str,
        token: &Token,
    ) -> ActionHandlerResponse<()> {
        self.update_item("pause_end", id, BodyEnd { end }, token)
    }
}
