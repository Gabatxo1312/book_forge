use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenLibraryApiBooks {
    docs: Vec<OpenLibraryApiBook>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenLibraryApiBook {
    first_publish_year: Option<i16>,
    title: String,
    author_name: Option<Vec<String>>,
    cover_edition_key: Option<String>,
}

const OPEN_LIBRARY_LINK: &str = "https://openlibrary.org/search.json";

pub async fn get_book_from_api(
    query: Option<&String>,
) -> Result<OpenLibraryApiBooks, reqwest::Error> {
    let formated_query = match query {
        Some(query) => query.replace(" ", "+"),
        None => String::from(""),
    };

    let request_url = format!("{}?q={}", OPEN_LIBRARY_LINK, formated_query);

    info!("Send request to OpenLibrary API endpoint : {}", request_url);

    let response = reqwest::get(format!("{}?q={}", OPEN_LIBRARY_LINK, formated_query))
        .await?
        .json()
        .await?;

    Ok(response)
}
