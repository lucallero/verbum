// pub struct BibleVerse {
//     pub book: String,
//     pub chapter: u32,
//     pub verse: u32,
//     pub text: String,
// }

pub struct Verse {
    pub number: u32,
    pub text: String,
}

pub struct Chapter {
    pub number: u32,
    pub verses: Vec<Verse>,
}
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
