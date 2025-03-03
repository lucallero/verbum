// pub struct BibleVerse {
//     pub book: String,
//     pub chapter: u32,
//     pub verse: u32,
//     pub text: String,
// }
use serde::Deserialize;
use std::collections::HashMap;

pub mod chapter;
pub mod verse;

use chapter::Chapter;

pub struct Book {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

pub struct Section {
    pub title: String,
    pub description: String,
    pub books: Vec<Book>,
}

pub struct Division {
    pub title: String,
    pub sections: Vec<Section>,
}

pub struct Bible {
    pub title: String,
    pub version: String,
}

//Metadata structures
#[derive(Debug, Deserialize)]
pub struct MetaBible {
    pub books: HashMap<String, MetaBook>,
}

#[derive(Debug, Deserialize)]
pub struct MetaBook {
    pub chapters: HashMap<u32, u32>,
}
