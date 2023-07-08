use fetch::ResponseHandler;
use prompt::{error_text, prompt_command, prompt_login, success_text};

mod auth;
mod block;
mod error;
mod fetch;
mod prompt;

fn main() {
    let mut authorizer = auth::Authorizer::new();
    let action_handler = fetch::ActionHandler::new();

    loop {
        if authorizer.login_necessary() {
            let (email, password) = prompt_login();
            match authorizer.login(email, password) {
                Ok(_) => println!("{}", success_text("> Anmeldung erfolgreich")),
                Err(_) => println!("{}", error_text("> Anmeldung fehlgeschlagen")),
            }
        } else {
            let token = authorizer.token().expect("> Kein Token gefunden");
            let command = prompt_command();

            match command.trim() {
                "start block" => {
                    action_handler
                        .start_block(token)
                        .handle_response("> Block getartet", "> Block bereits aktiv");
                }
                "end block" => {
                    action_handler
                        .end_block(token)
                        .handle_response("> Block beendet", "> Kein Block aktiv/Pause noch aktiv");
                }
                "start pause" => {
                    action_handler
                        .start_pause(token)
                        .handle_response("> Pause gestartet", "> Pause bereits aktiv");
                }
                "end pause" => {
                    action_handler
                        .end_pause(token)
                        .handle_response("> Pause beendet", "> Keine Pause aktiv");
                }
                "current" => {
                    action_handler
                        .get_current_block(token)
                        .handle_response("> Aktueller Block", "> Kein Block aktiv");
                }
                "all" => {
                    action_handler
                        .get_all_blocks(token)
                        .handle_response("> Alle Blocks", "> Keine Blocks");
                }
                "exit" => {
                    println!("{}", success_text("> Programm beendet"));
                    break;
                }
                _ => {
                    println!("{}", error_text("> Unbekanntes Kommando"));
                }
            }
        }
    }
}
