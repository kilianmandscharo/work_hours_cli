use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Block {
    id: i32,
    start: String,
    end: String,
    pauses: Option<Vec<Pause>>,
}

#[derive(Deserialize, Debug)]
pub struct Pause {
    id: i32,
    start: String,
    end: String,
}
