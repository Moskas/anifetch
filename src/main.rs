use colored::Colorize;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

// Query to use in request
const QUERY: &str = "
{
  User(name: \"Moskas\") {
      name
        statistics {
        manga {
          chaptersRead
          volumesRead
        }
      }
    }
}
";

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
  data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
  #[serde(rename = "User")]
  user: User,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
  name: String,
  statistics: Statistics,
}

#[derive(Debug, Serialize, Deserialize)]
struct Statistics {
  manga: Manga,
}

#[derive(Debug, Serialize, Deserialize)]
struct Manga {
  #[serde(rename = "chaptersRead")]
  chapters_read: u32,
  #[serde(rename = "volumesRead")]
  volumes_read: u32,
}

#[tokio::main]
async fn main() {
  let client = Client::new();
  // Define query and variables
  let json = json!({"query": QUERY, "user":{ "name": "Moskas"}});
  // Make HTTP post request
  let resp = client
    .post("https://graphql.anilist.co/")
    .header("Content-Type", "application/json")
    .header("Accept", "application/json")
    .body(json.to_string())
    .send()
    .await;

  let response_data: ResponseData;
  match resp {
    Ok(response) => {
      let text = response.text().await;
      match text {
        Ok(body) => {
          let result: Result<ResponseData, serde_json::Error> = serde_json::from_str(&body);
          match result {
            Ok(data) => {
              response_data = data;
            }
            Err(e) => {
              eprintln!("Error deserializing the response: {:?}", e);
              return;
            }
          }
        }
        Err(e) => {
          eprintln!("Error reading response text: {:?}", e);
          return;
        }
      }
    }
    Err(e) => {
      eprintln!("Error making the request: {:?}", e);
      return;
    }
  }

  //println!("{:#?}", response_data.data.user);
    println!("{}@anilist.co", response_data.data.user.name.blue());
    println!("{}","Manga stats".to_string().bold());
    println!("Chapters read: {}", response_data.data.user.statistics.manga.chapters_read.to_string().yellow());
    println!("Volumes read: {}", response_data.data.user.statistics.manga.volumes_read.to_string().yellow());
}
