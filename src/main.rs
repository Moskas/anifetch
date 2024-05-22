use colored::Colorize;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

mod cli;

// Query to use in request
const QUERY: &str = "
{
  User(name: \"$name\") {
      name
        statistics {
        manga {
          count
          chaptersRead
          volumesRead
          genres {
            genre
          }
          statuses {
            status
            count
          }
        }
        anime {
          count
          minutesWatched
          episodesWatched
          genres {
            genre
          }
          statuses {
            status
            count
          }
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
  anime: Anime,
}

#[derive(Debug, Serialize, Deserialize)]
struct Genre {
  genre: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Statuses {
  status: String,
  count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Manga {
  #[serde(rename = "chaptersRead")]
  chapters_read: u32,
  #[serde(rename = "volumesRead")]
  volumes_read: u32,
  count: u32,
  statuses: Vec<Statuses>,
  genres: Vec<Genre>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Anime {
  #[serde(rename = "minutesWatched")]
  minutes_watched: u32,
  #[serde(rename = "episodesWatched")]
  episodes_watched: u32,
  count: u32,
  statuses: Vec<Statuses>,
  genres: Vec<Genre>,
}

#[tokio::main]
async fn main() {
  let client = Client::new();
  let name = &cli::get_name();
  // Define query and variables
  let json = json!({"query": QUERY.replace("$name", name)});
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

  //println!("{:#?}", response_data.data);
  let user = format!("{}@anilist.co", response_data.data.user.name.blue());
  let manga_stats = format!(
    "{}: {}",
    "Manga stats".to_string().bold(),
    response_data.data.user.statistics.manga.count.to_string()[..].magenta()
  );
  let chapters_read = format!(
    "- Chapters read: {}",
    response_data
      .data
      .user
      .statistics
      .manga
      .chapters_read
      .to_string()
      .yellow()
  );
  let volumes_read = format!(
    "- Volumes read: {}",
    response_data
      .data
      .user
      .statistics
      .manga
      .volumes_read
      .to_string()
      .yellow()
  );
  let anime_stats = format!(
    "{}: {}",
    "Anime stats".to_string().bold(),
    response_data.data.user.statistics.anime.count.to_string()[..].magenta()
  );
  let minutes_watched = format!(
    "- Minutes watched: {}",
    response_data
      .data
      .user
      .statistics
      .anime
      .minutes_watched
      .to_string()
      .yellow()
  );
  let episodes_watched = format!(
    "- Episodes watched: {}",
    response_data
      .data
      .user
      .statistics
      .anime
      .episodes_watched
      .to_string()
      .yellow()
  );
  let manga_statuses = format!(
    "- {}",
    &response_data
      .data
      .user
      .statistics
      .manga
      .statuses
      .iter()
      .map(|s| format!("{}: {}", s.status, s.count))
      .collect::<Vec<String>>()
      .join(" ")[..]
  );
  let manga_genres = format!(
    "- Top 5 genres: {}",
    response_data
      .data
      .user
      .statistics
      .manga
      .genres
      .iter()
      .map(|x| x.genre.to_string())
      .collect::<Vec<String>>()[0..5]
      .join(", ")
      .yellow()
  );
  let anime_statuses = format!(
    "- {}",
    &response_data
      .data
      .user
      .statistics
      .anime
      .statuses
      .iter()
      .map(|s| format!("{}: {}", s.status, s.count))
      .collect::<Vec<String>>()
      .join(" ")[..]
  );
  let anime_genres = format!(
    "- Top 5 genres: {}",
    response_data
      .data
      .user
      .statistics
      .anime
      .genres
      .iter()
      .map(|x| x.genre.to_string())
      .collect::<Vec<String>>()[0..5]
      .join(", ")
      .yellow()
  );
  let right = vec![
    "",
    &user,
    "",
    &manga_stats,
    &chapters_read,
    &volumes_read,
    &manga_genres,
    &manga_statuses,
    "",
    &anime_stats,
    &episodes_watched,
    &minutes_watched,
    &anime_genres,
    &anime_statuses,
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
  ];
  let ascii = "⠀⠀⠠⢽⣝⣗⡽⡽⣝⢮⣪⣫⠀⠀⡠⡳⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠠⠑⢗⣗⡿⡽⣝⣗⡧⣣⠨⠐⠀⢀⠠⡂⡢⠨⡈⡂⠅⣂⣐⢄⠀⠂⠀⠁⢈⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⡀⣄⣢⢯⡫⡝⡬⣷⣟⡆⠅⢅⢆⢇⢇⢇⠪⡨⡰⢘⠨⢀⠄⠅⡑⠔⡄⡐⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀
⡲⣕⢧⢳⣝⡷⣽⢞⣽⢾⠙⢌⢜⢎⢎⢎⢎⢢⢕⢕⠌⢆⠪⡐⢌⠢⠂⠡⠘⡔⡀⠀⠈⠄⠀⠀⠂⠀⠨⡂
⡯⡪⡮⡗⣗⢽⡹⣹⣪⡇⡪⡎⡧⡫⡎⡇⡕⣕⢕⢕⠕⡅⡣⢪⢘⢌⢪⢐⡑⢌⠆⡡⠀⠌⠠⠁⠀⠁⡜⠀
⡏⡮⡮⡯⡪⡣⡋⡮⣞⠬⡮⣳⢹⢜⢎⢎⢞⢜⢼⢸⢪⢪⢪⢪⢢⢱⢨⢢⠪⡢⡣⡪⡠⠁⠌⡠⠀⣕⠍⠀
⣿⣺⢽⢽⡻⣞⣮⡯⡏⡯⡯⣪⢇⢧⠣⡣⡳⣙⢎⢗⡕⡧⣫⢪⢪⡪⡪⣒⢭⢪⢪⢪⠢⠠⢡⡠⡞⡕⠀⠀
⠃⢟⡽⣗⡯⣟⣗⠏⢕⢹⢝⣾⢱⢕⢇⠗⡳⢕⢇⢧⠳⡹⡸⡜⡎⣞⢼⢸⢜⢜⡜⡵⡹⡈⢮⡫⠊⠀⠀⠀
⠑⠐⠙⠵⡟⡿⡢⠡⡁⡂⡢⠉⡞⡜⠨⠘⠨⡊⠘⠢⠊⠪⢣⢣⢣⢳⢕⢧⡫⢮⡪⣳⢱⢡⡳⡄⠀⠀⠀⠀
⠀⠠⠀⠀⠢⢨⠨⡂⣖⡐⡀⠡⢸⠱⡡⠈⠄⠀⡀⠀⢀⠀⢀⢈⢬⠪⣎⢎⢎⢧⡫⡮⡣⡢⡯⡺⣢⡀⠀⠀
⠈⠀⠄⠀⠥⡑⢌⠐⡮⣳⢕⡄⢕⠅⠀⠐⠀⠁⠀⠀⠀⡀⠄⢐⢁⠳⠈⡎⣪⢧⡫⣞⢜⣎⢧⡣⡓⠁⠀⠀
⠄⠐⢀⠕⡑⢌⢐⠰⣝⢮⢇⡇⢕⠅⠠⠀⠀⠂⡀⢀⠁⠀⠀⡀⠠⠀⣐⢜⣕⢗⡽⣸⣺⡪⣗⢯⠢⠀⠀⠀
⠡⠈⠄⠂⡐⠰⠈⡼⡵⡝⡧⡇⠨⡂⡂⠀⠐⠀⠀⠀⠀⠂⠀⠀⡀⡱⡕⡷⡕⡏⡊⡪⢊⠊⠊⠁⠀⠀⠀⠀
⢈⠀⡂⠅⠀⡁⡔⣯⢺⢝⣎⢗⠡⢑⢀⠢⢀⠂⠈⠀⠐⠀⣀⡡⣢⡫⡎⣇⣟⡎⢌⠂⢑⠐⠀⠄⠁⠀⠀⠀
⠂⡂⡠⢠⣲⣞⢧⣣⢇⡟⣎⢂⢂⢂⠂⡂⠂⠆⢖⢮⢪⡳⡕⣝⡼⣮⡺⣜⢮⣗⢔⠡⠡⢂⢁⠈⢀⠐⠀⠀
⡀⡂⢊⢲⣳⢯⣟⡮⡯⣊⢐⠜⡐⢅⠂⠄⢅⢑⢱⢕⣇⣃⣯⣳⢽⣳⢝⢮⡳⡯⠂⠑⡑⢅⡂⠑⠠⠂⠄⠈
⠂⠀⡐⡽⣞⣟⣾⢽⡽⡒⣕⡕⠾⡎⡆⡱⠰⠁⠀⡧⣗⣟⡮⣯⣻⡺⡀⠑⠽⡈⠀⠀⠌⠰⠘⠌⠀⠄⠑⢀
⢀⠰⣜⣯⣟⡾⣽⢋⢪⢜⣆⣯⣞⢌⠪⡈⠀⠠⠀⢳⢯⡷⣽⡺⡺⠈⠄⠀⠀⠁⠂⠀⠀⠀⠁⠁⠂⠄⠀⠀
⡀⣮⣻⣞⡾⣝⡦⣺⣞⣷⡳⣗⢽⣆⠂⠀⠄⠠⠐⠸⣹⡽⡵⣗⠠⠈⡀⠀⠠⠀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀
⢜⣗⣗⡯⠋⢀⢟⡗⣗⣵⠟⠍⠙⠘⡢⠁⢀⠀⠀⢀⢳⢟⣯⣧⡀⠂⢀⠐⠀⠀⠄⠠⢁⠢⠐⠀⠀⠀⠀⠀";

  for (ascii_line, text_line) in ascii.lines().zip(right.iter()) {
    println!("{} {}", ascii_line, text_line);
  }
}
