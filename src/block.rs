use chrono::{DateTime, FixedOffset};
use colored::Colorize;
use serde::Deserialize;

use crate::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct Block {
    pub id: i32,
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

pub fn visualize_blocks(blocks: Vec<Block>) {
    if blocks.len() == 0 {
        return;
    }

    let min_start_block = blocks
        .iter()
        .min_by(|a, b| {
            DateTime::parse_from_rfc3339(&a.start)
                .unwrap()
                .time()
                .cmp(&DateTime::parse_from_rfc3339(&b.end).unwrap().time())
        })
        .unwrap();

    let max_end_block = blocks
        .iter()
        .max_by(|a, b| {
            DateTime::parse_from_rfc3339(&a.end)
                .unwrap()
                .time()
                .cmp(&DateTime::parse_from_rfc3339(&b.end).unwrap().time())
        })
        .unwrap();

    let min_start = DateTime::parse_from_rfc3339(&min_start_block.start).unwrap();
    let max_end = DateTime::parse_from_rfc3339(&max_end_block.end).unwrap();

    let total_minutes = max_end
        .time()
        .signed_duration_since(min_start.time())
        .num_minutes();
    let (w, _) = term_size::dimensions().unwrap();
    let minutes_per_space = (total_minutes as f64 / w as f64).ceil() as i64;

    for block in blocks {
        let start = DateTime::parse_from_rfc3339(&block.start).unwrap();
        let end = DateTime::parse_from_rfc3339(&block.end).unwrap();
        let duration = end.signed_duration_since(start);

        let minutes_to_min_start = start
            .time()
            .signed_duration_since(min_start.time())
            .num_minutes();

        let space_to_start = minutes_to_min_start / minutes_per_space;
        let space_from_start_to_end = duration.num_minutes() / minutes_per_space - 9;

        let block_title = format!("{} - {}", block.id, start.format("%d.%m.%Y"));

        match block.pauses {
            Some(pauses) => {
                let mut points: Vec<DateTime<FixedOffset>> = Vec::new();

                points.push(start);

                for pause in pauses {
                    points.push(DateTime::parse_from_rfc3339(&pause.start).unwrap());
                    points.push(DateTime::parse_from_rfc3339(&pause.end).unwrap());
                }

                points.push(end);

                for i in 1..points.len() {
                    let spaces = points[i].signed_duration_since(points[i - 1]).num_minutes()
                        / minutes_per_space;

                    let duration_minutes = duration.num_minutes() / minutes_per_space;
                    let percentage_of_whole: f64 = spaces as f64 / duration_minutes as f64;

                    let extra_spaces = (percentage_of_whole * 8 as f64).floor() as i64;

                    if i == 1 {
                        print!(
                            "{}{}{}",
                            empty_space(space_to_start),
                            block_title.on_green().black(),
                            empty_space(spaces + extra_spaces - block_title.len() as i64)
                                .on_green(),
                        );
                    } else {
                        if i % 2 == 0 {
                            print!("{}", empty_space(spaces + extra_spaces).on_blue());
                        } else {
                            print!("{}", empty_space(spaces + extra_spaces).on_green());
                        }
                    }
                }

                println!();

                println!(
                    "{}{}{}{}",
                    empty_space(space_to_start),
                    start.format("%H:%M:%S"),
                    empty_space(space_from_start_to_end),
                    end.format("%H:%M:%S")
                );
            }
            None => {
                println!(
                    "{}{}{}",
                    empty_space(space_to_start),
                    block_title.on_green().black(),
                    empty_space(space_from_start_to_end + 16 - block_title.len() as i64).on_green(),
                );

                println!(
                    "{}{}{}{}",
                    empty_space(space_to_start),
                    start.format("%H:%M:%S"),
                    empty_space(space_from_start_to_end),
                    end.format("%H:%M:%S")
                );
            }
        }

        println!();
    }
}

fn empty_space(n: i64) -> String {
    " ".repeat(n as usize)
}

#[derive(Deserialize, Debug, Clone)]
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
