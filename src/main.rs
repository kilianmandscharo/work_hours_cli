use std::io::stdin;

mod auth;
mod block;
mod fetch;

fn main() {
    let mut authorizer = auth::Authorizer::new();
    let handler = fetch::Handler::new();

    loop {
        if authorizer.login_necessary() {
            println!("Gib deine Email-Adresse ein:");
            let mut email = String::new();
            stdin()
                .read_line(&mut email)
                .expect("Fehler beim Lesen des Inputs");

            println!("Gib dein Passwort ein:");
            let mut password = String::new();
            stdin()
                .read_line(&mut password)
                .expect("Fehler beim Lesen des Inputs");

            let email = email.trim().to_string();
            let password = password.trim().to_string();

            if let Ok(_) = authorizer.login(email, password) {
                println!("Anmeldung erfolgreich");
            } else {
                println!("Anmeldung fehlgeschlagen");
            }
        } else {
            println!("Gib ein Kommando ein:");

            let mut command = String::new();
            stdin()
                .read_line(&mut command)
                .expect("Fehler beim Lesen des Inputs");

            match command.trim() {
                "start block" => {
                    let token = authorizer.token().unwrap();
                    match handler.start_block(token) {
                        Ok(status) => println!("{}", status),
                        Err(_) => println!("Fehler"),
                    }
                }
                "start pause" => {
                    let token = authorizer.token().unwrap();
                    match handler.start_pause(token) {
                        Ok(status) => println!("{}", status),
                        Err(_) => println!("Fehler"),
                    }
                }
                "end block" => {
                    let token = authorizer.token().unwrap();
                    match handler.end_block(token) {
                        Ok(status) => println!("{}", status),
                        Err(_) => println!("Fehler"),
                    }
                }
                "end pause" => {
                    let token = authorizer.token().unwrap();
                    match handler.end_pause(token) {
                        Ok(status) => println!("{}", status),
                        Err(_) => println!("Fehler"),
                    }
                }
                "current" => {
                    let token = authorizer.token().unwrap();
                    match handler.get_current_block(token) {
                        Ok(block) => println!("{:?}", block),
                        Err(err) => println!("{}", err),
                    }
                }
                "all" => {
                    let token = authorizer.token().unwrap();
                    match handler.get_all_blocks(token) {
                        Ok(blocks) => println!("{:?}", blocks),
                        Err(err) => println!("{}", err),
                    }
                }
                _ => {
                    println!("Unbekanntes Kommando")
                }
            }
        }
    }
}
