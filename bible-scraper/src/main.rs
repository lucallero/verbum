use tokio::{self};

pub mod model;
pub mod the_new_american_bible;

#[tokio::main]
async fn main() {
    scrape_bible().await;
}

async fn scrape_bible() {
    the_new_american_bible::scrape().await;
}
