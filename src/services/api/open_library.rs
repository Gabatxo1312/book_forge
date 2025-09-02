use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiBooks {
    docs: Vec<ApiBook>
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiBook {
    first_publish_year: Option<i16>,
    title: String,
    author_name: Option<Vec<String>>,
    cover_edition_key: Option<String>
}

const OPEN_LIBRARY_LINK: &str = "https://openlibrary.org/search.json";

pub async fn get_book_from_api(query: Option<&String>) -> Result<ApiBooks, reqwest::Error>  {
    let formated_query = match query {
        Some(query) => query,
        None => &String::from("")
    };

    let response = reqwest::get(format!("{}?q={}", OPEN_LIBRARY_LINK, formated_query))
        .await?
        .json()
        .await?;

    Ok(response)
}
