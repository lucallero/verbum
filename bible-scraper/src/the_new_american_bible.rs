use reqwest::Client;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::fs;

use crate::models::verse::Verse;
use crate::models::MetaBible;

async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    let response = Client::new().get(url).send().await?;
    let text = response.text().await?;
    Ok(text)
}

pub async fn scrape() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.vatican.va/archive/ENG0839/__P3.HTM"; // URL of the page

    // metadata
    let meta_bible = load_metadata()?;

    // getting metadata for Genesis
    let meta_book = get_book_metadata(&meta_bible, "Genesis").ok_or("Book not found")?;
    let mut chapters = meta_book.chapters.keys().collect::<Vec<&u32>>();

    chapters.sort();
    println!("{:?}", chapters);

    let response = fetch(url).await?;
    let document = Document::from(response.as_str());

    // let verses = scrape_verses(document);

    for chapter in chapters {
        //only chapter 1 atm
        if chapter == &1 {
            // loop over metadata
            let verses = meta_book.chapters.get(chapter).unwrap();

            //TODO: find a way to discover which is the next page
            // make use of the metadata to know the next page
            // then concatenate the chapter number to assemble the URL
            // let url = format!("https://www.vatican.va/archive/ENG0839/__P3.HTM{}", chapter);

            for node in document.find(Name("p").and(Class("MsoNormal"))) {
                let mut verse: Verse = Verse::new(0, "".to_string());

                let text = node.text().trim().to_string();

                let verse_number = scrape_verse_number(&text);
                match verse_number {
                    Ok(verse_number) => {
                        verse.number = verse_number;
                    }
                    Err(_) => {
                        verse.text = scrape_verse_text(&text);
                    }
                }
                println!("{:?}", verse);
            }
        }
    }

    // println!("{:?}", verses);

    println!("Scraping done!");
    Ok(())
}

// fn initialize_data_structure() ->

// Loop over paragraphs with class "MsoNormal" to extract the verses, number only
// fn scrape_verses(document: Document) -> Vec<u32> {
//     let mut verses = Vec::new();
//     for node in document.find(Name("p").and(Class("MsoNormal"))) {
//         let text = node.text().trim().to_string();
//         let verse_number = scrape_verse_number(&text);
//         match verse_number {
//             Ok(verse_number) => verses.push(verse_number),
//             Err(_) => continue,
//         }
//     }
//     verses
// }

// fn scrape_book_title(text: &str) -> String {
//     text.trim().to_string()
// }

fn scrape_verse_number(text: &str) -> Result<u32, std::num::ParseIntError> {
    let parsed = text.parse::<u32>()?;
    Ok(parsed)
}

fn scrape_verse_text(text: &str) -> String {
    text.trim().to_string()
}

fn load_metadata() -> Result<MetaBible, Box<dyn std::error::Error>> {
    let yaml_content = fs::read_to_string(
        "/home/lucallero/Rust/verbum/bible-scraper/src/the_new_american_bible/books.yaml",
    )?;
    let bible: MetaBible = serde_yaml::from_str(&yaml_content)?;

    // println!("{:#?}", bible);
    Ok(bible)
}

fn get_book_metadata<'a>(
    meta_bible: &'a MetaBible,
    book_title: &'a str,
) -> Option<&'a crate::models::MetaBook> {
    meta_bible.books.get(book_title)
}
