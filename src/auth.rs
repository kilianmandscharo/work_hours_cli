use crate::error::AuthError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const TOKEN_DURATION: Duration = Duration::from_secs(10 * 60);
const DATA_FILE: &str = "token.json";
const DATA_DIR: &str = ".work_hours_cli_data";
const LOGIN_URL: &str = "http://localhost:8080/login";

#[derive(Serialize, Deserialize)]
pub struct Token {
    jwt: String,
    expires_at: std::time::SystemTime,
}

#[derive(Serialize)]
struct Login {
    email: String,
    password: String,
}

pub struct Authorizer {
    token: Option<Token>,
    pub token_path: String,
}

impl Authorizer {
    pub fn new() -> Authorizer {
        let token_path = create_data_dir().expect("Fehler beim Erstellen des Datenordners");

        let token: Option<Token> = match load_token_from_file(&token_path) {
            Ok(token) => Some(token),
            _ => None,
        };

        Authorizer { token, token_path }
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

        save_token_to_file(&token, &self.token_path)?;
        self.token = Some(token);

        Ok(())
    }

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

fn save_token_to_file(token: &Token, path: &str) -> Result<(), std::io::Error> {
    let token = serde_json::to_string(&token)?;
    std::fs::write(path, token)?;
    Ok(())
}

fn load_token_from_file(path: &str) -> Result<Token, AuthError> {
    let token = String::from_utf8(std::fs::read(path)?)?;
    let token = serde_json::from_str(&token)?;
    Ok(token)
}

fn create_data_dir() -> Result<String, std::io::Error> {
    let home_dir = home::home_dir().expect("Fehler beim Ermitteln des 'home'-Ordners");
    let home_dir = home_dir
        .into_os_string()
        .into_string()
        .expect("Fehler beim Ermitteln des 'home'-Ordners");

    if let Err(err) = std::fs::create_dir_all(format!("{home_dir}/{DATA_DIR}")) {
        return Err(err);
    }

    let token_path = format!("{home_dir}/{DATA_DIR}/{DATA_FILE}");

    Ok(token_path)
}

// #[test]
// fn token_test() {
//     let jwt = String::from("test_token_content");
//     let token = Token::new(jwt);
//     let _ = save_token_to_file(&token).unwrap();
//     let token = load_token_from_file().unwrap();
//     assert_eq!("test_token_content", token.jwt);
//     assert_eq!(false, token.has_expired());
// }
