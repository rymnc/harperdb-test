use harperdb::{ HarperConfig, Harper };
use harperdb as harper;
use serde::{Deserialize, Serialize};
use std::{error::Error};
use std::env::var;
use lazy_static::lazy_static;

lazy_static! {
    static ref PASSWORD: String = var("HARPERDB_PASS").unwrap();
    static ref USER: String = var("HARPERDB_USER").unwrap();
    static ref URL: String = var("HARPERDB_URL").unwrap();
}

#[macro_use]
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct HorrorMovieRecord{
    id: String,
    jump_scares: Option<u8>,
    bg_music_rating: Option<u8>,
    __createdtime__: usize,
    __updatedtime__: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = "movies";
    let config: HarperConfig = HarperConfig {
        url: &*URL,
        username: &*USER,
        password: &*PASSWORD,
        schema,
    };
    let harper_client = Harper::new(config);
    let insert_option: harper::QueryOptions = harper::QueryOptions {
        table: "horror",
        schema: "movies",
        records:json!([
            {
            "id": "test-1234",
            "jump_scares": 12,
            "bg_music_rating": 9
            },
            {
                "id": "test-5678",
                "jump_scares": 9,
                "bg_music_rating": 8,
            },
            {
                "id": "scary-1234",
                "jump_scares": 10,
                "bg_music_rating": 12
            }
        ]),
    };
    harper_client.insert(insert_option).await?;

    let update_option: harper::QueryOptions = harper::QueryOptions{
        table: "horror",
        schema: "movies",
        records: json!([{
            "id": "scary-1234",
            "jump_scares": 24
        }])
    };

    let res = harper_client.update(update_option).await?;

    let result = harper_client.query("SELECT * FROM movies.horror WHERE bg_music_rating >=9").await?;

    let records: Vec<HorrorMovieRecord> = result.json().await?;

    println!("{:#?}", records);

    Ok(())
}
