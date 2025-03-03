// use the_new_american_bible;
use tokio::{self};

pub mod models;
pub mod the_new_american_bible;

#[tokio::main]
async fn main() {
    scrape_bible().await;
}

async fn scrape_bible() {
    let _ = the_new_american_bible::scrape().await;
}
