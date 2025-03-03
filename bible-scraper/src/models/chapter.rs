use super::verse::Verse;

pub struct Chapter {
    pub number: u32,
    pub verses: Vec<Verse>,
}

impl Chapter {
    pub fn new(number: u32, verses: Vec<Verse>) -> Self {
        Self { number, verses }
    }
}
