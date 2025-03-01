use reqwest::Client;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::fs;

use crate::model::MetaBible;

pub async fn scrape() {
    let url = "https://www.vatican.va/archive/ENG0839/__P3.HTM"; // URL of the page

    // fetching the page
    let response = Client::new()
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // loading metadata from books.yaml
    let meta_bible = load_metadata().unwrap();

    let meta_book = get_book_metadata(&meta_bible, "Genesis");
    println!("{:#?}", meta_book);

    let document = Document::from(response.as_str());

    let verses = scrape_verses(document);

    println!("{:?}", verses);

    println!("Scraping done!");
}

// Loop over paragraphs with class "MsoNormal" to extract the verses, number only
fn scrape_verses(document: Document) -> Vec<i32> {
    let mut verses = Vec::new();
    for node in document.find(Name("p").and(Class("MsoNormal"))) {
        let text = node.text().trim().to_string();
        let verse_number = scrape_verse_number(&text);
        match verse_number {
            Ok(verse_number) => verses.push(verse_number),
            Err(_) => continue,
        }
    }
    verses
}

fn scrape_book_title(text: &str) -> String {
    text.trim().to_string()
}

fn scrape_verse_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    let parsed = text.parse::<i32>()?;
    Ok(parsed)
}

fn scrape_verse_text(text: &str) -> String {
    text.trim().to_string()
}

fn load_metadata() -> Result<MetaBible, Box<dyn std::error::Error>> {
    let yaml_content = fs::read_to_string(
        "/Users/luciano/Rust/verbum/bible-scraper/src/the_new_american_bible/books.yaml",
    )?;
    let bible: MetaBible = serde_yaml::from_str(&yaml_content)?;

    // println!("{:#?}", bible);
    Ok(bible)
}

fn get_book_metadata<'a>(
    meta_bible: &'a MetaBible,
    book_title: &'a str,
) -> Option<&'a crate::model::MetaBook> {
    meta_bible.books.get(book_title)
}
