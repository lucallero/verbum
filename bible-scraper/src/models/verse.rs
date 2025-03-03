use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Verse {
    pub number: u32,
    pub text: String,
}

impl Verse {
    pub fn new(number: u32, text: String) -> Self {
        Self { number, text }
    }
}
