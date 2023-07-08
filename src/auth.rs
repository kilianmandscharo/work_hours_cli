use core::fmt;
use serde::{Deserialize, Serialize};
use std::{string::FromUtf8Error, time::Duration};

const TOKEN_DURATION: Duration = Duration::from_secs(10 * 60);
const FILE_NAME: &str = "token.json";
const LOGIN_URL: &str = "http://localhost:8080/login";

#[derive(Debug)]
pub enum AuthError {
    JSONError(serde_json::Error),
    HTTPError(reqwest::Error),
    FSError(std::io::Error),
    UTF8Error(FromUtf8Error),
    AuthError(u16),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthError::JSONError(err) => write!(f, "JSON error: {}", err),
            AuthError::HTTPError(err) => write!(f, "HTTP error: {}", err),
            AuthError::FSError(err) => write!(f, "File system error: {}", err),
            AuthError::UTF8Error(err) => write!(f, "Byte parsing error: {}", err),
            AuthError::AuthError(status) => write!(f, "Authentification error: {}", status),
        }
    }
}

impl From<serde_json::Error> for AuthError {
    fn from(error: serde_json::Error) -> Self {
        AuthError::JSONError(error)
    }
}

impl From<reqwest::Error> for AuthError {
    fn from(error: reqwest::Error) -> Self {
        AuthError::HTTPError(error)
    }
}

impl From<std::io::Error> for AuthError {
    fn from(error: std::io::Error) -> Self {
        AuthError::FSError(error)
    }
}

impl From<FromUtf8Error> for AuthError {
    fn from(error: FromUtf8Error) -> Self {
        AuthError::UTF8Error(error)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    jwt: String,
    expires_at: std::time::SystemTime,
}

#[derive(Serialize, Deserialize)]
struct Login {
    email: String,
    password: String,
}

pub struct Authorizer {
    token: Option<Token>,
}

impl Authorizer {
    pub fn new() -> Authorizer {
        let token: Option<Token> = match load_token_from_file() {
            Ok(token) => Some(token),
            _ => None,
        };
        Authorizer { token }
    }

    pub fn token(&self) -> Option<&Token> {
        if let Some(ref token) = self.token {
            return Some(token);
        } else {
            return None;
        }
    }

    pub fn login(&mut self, email: String, password: String) -> Result<(), AuthError> {
        if !self.login_necessary() {
            return Ok(());
        }

        let login = Login { email, password };
        let login = serde_json::to_string(&login)?;

        let client = reqwest::blocking::Client::new();
        let res = client.post(LOGIN_URL).body(login).send()?;

        let status = res.status();
        if !res.status().is_success() {
            return Err(AuthError::AuthError(status.as_u16()));
        }

        let token = res.text()?;
        let token = Token::new(String::from(token));

        save_token_to_file(&token)?;
        self.token = Some(token);

        Ok(())
    }

    // fn refresh_token(&mut self, token: &Token) -> Result<(), AuthError> {
    //     if !self.login_necessary() {
    //         return Ok(());
    //     }
    //
    //     let client = reqwest::blocking::Client::new();
    //     let res = client
    //         .post(REFRESH_URL)
    //         .header("Authorization", format!("Bearer {}", token.jwt))
    //         .send()?;
    //
    //     let status = res.status();
    //     if !res.status().is_success() {
    //         return Err(AuthError::AuthError(status.as_u16()));
    //     }
    //
    //     let token = res.text()?;
    //     let token = Token::new(String::from(token));
    //
    //     save_token_to_file(&token)?;
    //     *self.token.lock().unwrap() = Some(token);
    //
    //     Ok(())
    // }

    pub fn login_necessary(&self) -> bool {
        if let Some(ref token) = self.token {
            if token.has_expired() {
                return true;
            } else {
                return false;
            }
        } else {
            true
        }
    }
}

impl Token {
    fn new(jwt: String) -> Token {
        let expires_at = std::time::SystemTime::now() + TOKEN_DURATION;
        Token { jwt, expires_at }
    }

    fn has_expired(&self) -> bool {
        let now = std::time::SystemTime::now();
        if let Ok(_) = self.expires_at.duration_since(now) {
            false
        } else {
            true
        }
    }

    pub fn token_string(&self) -> &str {
        &self.jwt
    }
}

fn save_token_to_file(token: &Token) -> Result<(), std::io::Error> {
    let token = serde_json::to_string(&token)?;
    std::fs::write(FILE_NAME, token)?;
    Ok(())
}

fn load_token_from_file() -> Result<Token, AuthError> {
    let token = String::from_utf8(std::fs::read(FILE_NAME)?)?;
    let token = serde_json::from_str(&token)?;
    Ok(token)
}

#[test]
fn token_test() {
    let jwt = String::from("test_token_content");
    let token = Token::new(jwt);
    let _ = save_token_to_file(&token).unwrap();
    let token = load_token_from_file().unwrap();
    assert_eq!("test_token_content", token.jwt);
    assert_eq!(false, token.has_expired());
}
