use block::visualize_blocks;
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
    let mut action_handler = fetch::ActionHandler::new();

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
                Command::BlockStart(homeoffice) => {
                    action_handler
                        .start_block(token, homeoffice)
                        .handle_response("> Block gestartet", "> Block bereits aktiv");
                }
                Command::BlockEnd => {
                    action_handler
                        .end_block(token)
                        .handle_response("> Block beendet", "> Kein Block aktiv/Pause noch aktiv");
                }
                Command::PauseStart => {
                    action_handler
                        .start_pause(token)
                        .handle_response("> Pause gestartet", "> Pause bereits aktiv");
                }
                Command::PauseEnd => {
                    action_handler
                        .end_pause(token)
                        .handle_response("> Pause beendet", "> Keine Pause aktiv");
                }
                Command::BlockCurrent => {
                    let block = action_handler
                        .get_current_block(token)
                        .handle_response("> Aktueller Block", "> Kein Block aktiv");

                    if let Some(block) = block {
                        block.display();
                    }
                }
                Command::BlockAll => {
                    let blocks = action_handler
                        .get_all_blocks(token)
                        .handle_response("> Alle Blöcke", "> Keine Blöcke");

                    if let Some(blocks) = blocks {
                        visualize_blocks(blocks);
                    }
                }
                Command::BlockDelete(id) => {
                    action_handler
                        .delete_block(id, token)
                        .handle_response("Block gelöscht", "Block nicht gefunden");
                }
                Command::BlockUpdateStart((id, start)) => {
                    action_handler
                        .update_block_start(id, &start, token)
                        .handle_response("Block angepasst", "Fehler beim Anpassen");
                }
                Command::BlockUpdateEnd((id, end)) => {
                    action_handler
                        .update_block_end(id, &end, token)
                        .handle_response("Block angepasst", "Fehler beim Anpassen");
                }
                Command::BlockUpdateHomeoffice((id, homeoffice)) => {
                    action_handler
                        .update_block_homeoffice(id, homeoffice, token)
                        .handle_response("Block angepasst", "Fehler beim Anpassen");
                }
                Command::PauseDelete(id) => {
                    action_handler
                        .delete_pause(id, token)
                        .handle_response("Pause gelöscht", "Pause nicht gefunden");
                }
                Command::PauseUpdateStart((id, start)) => {
                    action_handler
                        .update_pause_start(id, &start, token)
                        .handle_response("Pause angepasst", "Fehler beim Anpassen");
                }
                Command::PauseUpdateEnd((id, end)) => {
                    action_handler
                        .update_pause_end(id, &end, token)
                        .handle_response("Pause angepasst", "Fehler beim Anpassen");
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
