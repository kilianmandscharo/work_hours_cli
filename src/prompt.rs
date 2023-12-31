use colored::{ColoredString, Colorize};
use std::io::stdin;

pub fn prompt_login() -> (String, String) {
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

    (email.trim().to_string(), password.trim().to_string())
}

pub fn prompt_command() -> String {
    println!("> Gib ein Kommando ein:");
    let mut command = String::new();
    stdin()
        .read_line(&mut command)
        .expect("Fehler beim Lesen des Inputs");

    command
}

pub fn error_text(s: &str) -> ColoredString {
    ColoredString::from(s).red()
}

pub fn success_text(s: &str) -> ColoredString {
    ColoredString::from(s).green()
}

// pub fn highlight_text(s: &str) -> ColoredString {
//     ColoredString::from(s).blue()
// }
