use harperdb::{ HarperConfig, Harper };
use harperdb as harper;
use serde::{Deserialize, Serialize};
use std::{error::Error};

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

fn getEnv(var: &str) -> &'static str {
    match option_env!(format!("{}", var)) {
        Some(s) =>s,
        None => panic!("Cannot find {} in ENV", var)
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = getEnv("HARPERDB_URL");
    let username = getEnv("HARPERDB_USER");
    let password = getEnv("HARPERDB_PASS");
    let schema = "movies";
    let config: HarperConfig = HarperConfig {
        url,
        username,
        password,
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
