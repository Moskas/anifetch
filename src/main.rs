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
  let detailed_watchtime =
    convert_minutes_to_days_hours_minutes(response_data.data.user.statistics.anime.minutes_watched);
  let minutes_watched = format!(
    "- Watchtime: {} days, {} hours, {} minutes",
    detailed_watchtime.0.to_string().bold(),
    detailed_watchtime.1.to_string().bold(),
    detailed_watchtime.2.to_string().bold()
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
  let manga_statuses = format_statuses(response_data.data.user.statistics.manga.statuses);
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
  let anime_statuses = format_statuses(response_data.data.user.statistics.anime.statuses);
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
  let stats = vec![
    &user,
    &manga_stats,
    &chapters_read,
    &volumes_read,
    &manga_genres,
    &manga_statuses,
    &anime_stats,
    &episodes_watched,
    &minutes_watched,
    &anime_genres,
    &anime_statuses,
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
⢜⣗⣗⡯⠋⢀⢟⡗⣗⣵⠟⠍⠙⠘⡢⠁⢀⠀⠀⢀⢳⢟⣯⣧⡀⠂⢀⠐⠀⠀⠄⠠⢁⠢⠐⠀⠀⠀⠀⠀
";

  if cli::use_ascii() && !(ascii.lines().count() < 1) {
    let mut ascii_lines = ascii.lines();
    let mut right_iter = stats.iter();

    // Print paired lines from both iterators
    for (ascii_line, text_line) in ascii_lines.by_ref().zip(right_iter.by_ref()) {
      println!("{} {}", ascii_line, text_line);
    }

    // Print remaining ascii lines if any
    for ascii_line in ascii_lines {
      println!("{}", ascii_line);
    }
  } else {
    for line in stats {
      println!("{}", line);
    }
  }
  //format_statuses(response_data.data.user.statistics.manga.statuses);
  //format_statuses(response_data.data.user.statistics.anime.statuses);
}

fn format_statuses(mut statuses: Vec<Statuses>) -> String {
  // For whatever reason anilist api returns differently ordered statuses for manga and anime
  // so for more coherent output we need to reorder elements to keep them inline
  statuses.sort_by(|a, b| a.status.cmp(&b.status));
  let mut formatted_statuses = vec![];
  for status in statuses {
    match &status.status[..] {
      "COMPLETED" => formatted_statuses.push(format!(
        "{} {}",
        titlecase(status.status),
        status.count.to_string()[..].green()
      )),
      "PAUSED" => formatted_statuses.push(format!(
        "{} {}",
        titlecase(status.status),
        status.count.to_string()[..].magenta()
      )),
      "CURRENT" => formatted_statuses.push(format!(
        "{} {}",
        titlecase(status.status),
        status.count.to_string()[..].yellow()
      )),
      "PLANNING" => formatted_statuses.push(format!(
        "{} {}",
        titlecase(status.status),
        status.count.to_string()[..].blue()
      )),
      "REPEATING" => formatted_statuses.push(format!(
        "{} {}",
        titlecase(status.status),
        status.count.to_string()[..].cyan()
      )),
      "DROPPED" => formatted_statuses.push(format!(
        "{} {}",
        titlecase(status.status),
        status.count.to_string()[..].red()
      )),
      _ => formatted_statuses.push(titlecase(status.status).into()),
    }
  }
  format!("- {}", formatted_statuses.join(", "))
}

fn titlecase(word: String) -> String {
  format!(
    "{}{}",
    word[0..1].to_uppercase(),
    word[1..].to_ascii_lowercase().to_string()
  )
}

fn convert_minutes_to_days_hours_minutes(minutes: u32) -> (u32, u32, u32) {
  let hours = minutes / 60;
  let days = hours / 24;
  (days, hours, minutes)
}
