use fetch::ResponseHandler;
use parse::{parse_command, Command};
use prompt::{error_text, prompt_command, prompt_login, success_text};

mod auth;
mod block;
mod error;
mod fetch;
mod parse;
mod prompt;
mod time;

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

            match parse_command(command.trim()) {
                Command::StartBlock => {
                    action_handler
                        .start_block(token)
                        .handle_response("> Block gestartet", "> Block bereits aktiv");
                }
                Command::StartBlockHomeoffice => {
                    action_handler
                        .start_block_homeoffice(token)
                        .handle_response("> Block gestartet", "> Block bereits aktiv");
                }
                Command::EndBlock => {
                    action_handler
                        .end_block(token)
                        .handle_response("> Block beendet", "> Kein Block aktiv/Pause noch aktiv");
                }
                Command::StartPause => {
                    action_handler
                        .start_pause(token)
                        .handle_response("> Pause gestartet", "> Pause bereits aktiv");
                }
                Command::EndPause => {
                    action_handler
                        .end_pause(token)
                        .handle_response("> Pause beendet", "> Keine Pause aktiv");
                }
                Command::Current => {
                    let block = action_handler
                        .get_current_block(token)
                        .handle_response("> Aktueller Block", "> Kein Block aktiv");

                    if let Some(block) = block {
                        block.display();
                    }
                }
                Command::All => {
                    let blocks = action_handler
                        .get_all_blocks(token)
                        .handle_response("> Alle Blöcke", "> Keine Blöcke");

                    if let Some(blocks) = blocks {
                        for block in blocks {
                            block.display();
                        }
                    }
                }
                Command::Delete(id) => {
                    action_handler
                        .delete_block(id, token)
                        .handle_response("Block gelöchst", "Block nicht gefunden");
                }
                Command::Exit => {
                    println!("{}", success_text("> Programm beendet"));
                    break;
                }
                Command::Unknown => {
                    println!("{}", error_text("Unbekanntes Kommando"))
                }
            }
        }
    }
}
