use serde::Deserialize;

use crate::time::Duration;

#[derive(Deserialize, Debug)]
pub struct Block {
    id: i32,
    start: String,
    end: String,
    pauses: Option<Vec<Pause>>,
    homeoffice: bool,
}

impl Block {
    pub fn display(&self) {
        if self.end.len() == 0 {
            let start = chrono::DateTime::parse_from_rfc3339(&self.start).unwrap();
            let now = chrono::Local::now();

            let d = Duration::from_chrono_duration(now.signed_duration_since(start));

            println!("Block {} - AKTIV", self.id);
            println!("Homeoffice: {}", self.homeoffice);
            println!("Aktiv seit: {}", start.format("%d.%m.%Y %H:%M:%S"));
            println!("Zeit: {:0>2}:{:0>2}:{:0>2}", d.hours, d.minutes, d.seconds);
        } else {
            let start = chrono::DateTime::parse_from_rfc3339(&self.start).unwrap();
            let end = chrono::DateTime::parse_from_rfc3339(&self.end).unwrap();

            let d = Duration::from_chrono_duration(end.signed_duration_since(start));

            println!("Block {} - ABGESCHLOSSEN", self.id);
            println!("Homeoffice: {}", self.homeoffice);
            println!("Start: {}", start.format("%d.%m.%Y %H:%M:%S"));
            println!("End: {}", end.format("%d.%m.%Y %H:%M:%S"));
            println!("Zeit: {:0>2}:{:0>2}:{:0>2}", d.hours, d.minutes, d.seconds);
        }

        if let Some(ref pauses) = self.pauses {
            for pause in pauses {
                pause.display();
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Pause {
    id: i32,
    start: String,
    end: String,
}

impl Pause {
    pub fn display(&self) {
        if self.end.len() == 0 {
            let start = chrono::DateTime::parse_from_rfc3339(&self.start).unwrap();
            let now = chrono::Local::now();

            let d = Duration::from_chrono_duration(now.signed_duration_since(start));

            println!("Pause {} - AKTIV", self.id);
            println!("Aktiv seit: {}", start.format("%d.%m.%Y %H:%M:%S"));
            println!("Zeit: {:0>2}:{:0>2}:{:0>2}", d.hours, d.minutes, d.seconds);
        } else {
            let start = chrono::DateTime::parse_from_rfc3339(&self.start).unwrap();
            let end = chrono::DateTime::parse_from_rfc3339(&self.end).unwrap();

            let d = Duration::from_chrono_duration(end.signed_duration_since(start));

            println!("Pause {} - ABGESCHLOSSEN", self.id);
            println!("Start: {}", start.format("%d.%m.%Y %H:%M:%S"));
            println!("End: {}", end.format("%d.%m.%Y %H:%M:%S"));
            println!("Zeit: {:0>2}:{:0>2}:{:0>2}", d.hours, d.minutes, d.seconds);
        }
    }
}
