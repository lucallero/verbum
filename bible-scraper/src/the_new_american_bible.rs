use reqwest::Client;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

enum BooksMetadata {
    Genesis { chapters: }
}

pub async fn scrape() {
    let url = "https://www.vatican.va/archive/ENG0839/__P3.HTM"; // URL of the page
    let response = Client::new()
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Document::from(response.as_str());

    // Targeting paragraphs with class "MsoNormal"
    for node in document.find(Name("p").and(Class("MsoNormal"))) {
        let text = node.text().trim().to_string();

        match text.parse::<i32>() {
            Ok(num) => println!("Parsed number: {}", num),
            Err(err) => println!("Failed to parse: {}:{}", err, text),
        }
        // println!("{}", text);
    }
    println!("Scraping done!");
}



fn scrape_verses(document: Document) -> Vec<i32> {
    let mut verses = Vec::new();

    for node in document.find(Name("p").and(Class("MsoNormal"))) {
        let text = node.text().trim().to_string();
        let verse_number = scrape_verse_number(&text);
        verses.push(verse_number);
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
