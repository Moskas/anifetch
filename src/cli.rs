use clap::Parser;
use clap_derive::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Anilist username that you want to look up
  #[arg(short, long)]
  username: String,
  /// Which media should be displayed: All, Anime, Manga
  #[arg(short, long, default_value = "All")]
  media: String,
  /// Disable ascii print
  #[arg(short, long, default_value_t = true, action = clap::ArgAction::SetFalse)]
  disable_ascii: bool,
  /// Load custom ascii from specified file
  #[arg(short, long, value_name = "CUSTOM_ASCII_PATH")]
  custom_ascii: Option<String>,
}

pub fn get_name() -> String {
  Args::parse().username
}

pub fn get_media() -> String {
  Args::parse().media
}

pub fn use_ascii() -> bool {
  Args::parse().disable_ascii
}

pub fn load_ascii() -> String {
  let default_ascii = "⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻⣟⣿⣻
⣯⣿⡽⣷⢿⣽⢷⡿⣽⡷⣿⡽⣷⢿⣽⢷⡿⣽⢷⡿⣽⢷⡿⣽⡷⣿⡽⣷⢿⣽⢷⡿⣽⡷⣿⡽⣷⢿⣽⣯
⣽⣾⣻⣟⣯⡿⣯⡿⣯⡿⣯⣿⣻⣯⡿⣯⣿⣻⣟⣿⣻⣟⡿⣯⡿⣯⣿⣻⣯⡿⣯⡿⣯⡿⣯⣿⣻⣯⣷⢿
⢷⣻⣽⣯⢿⣽⢿⣽⢿⣽⢿⣞⣿⣞⣿⣻⡾⣯⡿⣾⢯⣿⣽⣯⣿⣻⣾⣻⣾⣻⣯⡿⣯⣿⣻⣾⣻⡾⣯⣿
⣟⣯⣷⣟⣿⣽⢿⣽⢿⣽⢿⣽⡾⠯⠟⠷⠿⠯⠿⣻⢟⠷⠿⡞⡷⣿⢾⣻⣾⣻⣾⣻⣟⣾⣻⡾⣯⣿⣻⡾
⢿⣽⢷⣟⣷⣟⣿⣽⢿⣽⣟⣷⡟⠀⠀⠀⠀⠀⠀⠹⡸⡸⡱⡱⡱⣻⣟⣿⢾⣻⡾⣯⡿⣽⣯⢿⣻⡾⣯⣿
⢿⣽⢿⣽⢷⣟⣷⣟⣿⣳⣿⣳⠁⠀⠀⠀⠀⠀⠀⠀⢕⢕⢕⢕⢕⣿⣽⣾⢿⣯⡿⣯⣿⣻⡾⣿⢯⣿⣻⣾
⣯⡿⣯⣿⣻⣽⡷⣿⣽⣻⣾⠃⠀⠀⠀⠀⠀⠀⠀⠀⠐⢕⢕⢕⢕⢿⣺⣯⡿⣷⣟⣿⢾⣯⢿⣻⡿⣽⡷⣟
⣯⣿⣻⡾⣯⣷⢿⣻⣾⣻⡎⠀⠀⠀⠀⢀⡄⠀⠀⠀⠀⠘⡜⡜⠬⣿⣯⣷⢿⣻⣾⣻⣯⢿⣻⣯⡿⣯⣿⣻
⣷⣯⣷⡿⣿⢾⡿⣯⣷⠿⠀⠀⠀⠀⠀⣼⢷⠀⠀⠀⠀⠀⢱⠱⡍⣷⡿⣾⢿⣯⣷⢿⡾⣿⢯⣷⣿⣻⡾⣯
⣷⢿⣞⣿⣽⣟⣿⣽⣾⠃⠀⠀⠀⠀⢰⣟⣿⢇⠀⠀⠀⠀⠀⡫⡪⣟⣿⣽⣟⣷⣟⣿⣻⣟⣿⣳⣿⡽⣟⣯
⣽⢿⣽⢷⡿⣾⣻⡾⡇⠀⠀⠀⠀⠀⠈⠈⠈⠉⠀⠀⠀⠀⠀⠨⡪⣟⣷⢿⡾⣷⢿⣽⢷⡿⣽⣻⣾⣻⡿⣽
⣽⢿⣽⣟⣿⣽⣯⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢊⢟⢽⢻⢝⢟⢯⢻⢫⢿⣻⣽⡾⣯⣿⣻
⣽⣟⣷⣟⣷⢿⣺⠁⠀⠀⠀⠀⢀⣤⣤⣤⣤⣤⣤⡄⠀⠀⠀⠀⠀⢣⢣⢣⢣⢣⢣⢣⢣⠣⣿⣽⣟⣿⢾⣻
⣷⣟⣷⢿⣽⣟⠇⠀⠀⠀⠀⠀⣼⣯⡿⣾⢯⣷⣟⣷⠀⠀⠀⠀⠀⠈⡎⡎⡎⡎⡎⡎⡎⡎⣿⣾⣻⡾⣟⣯
⣽⣯⣟⣿⣳⡿⣶⢶⡶⣶⢶⡶⣟⣾⣟⣿⣻⡷⣿⣽⢷⡶⣶⡶⣶⣶⢾⡾⣞⣾⣞⣾⣞⣾⣻⡾⣯⣿⣻⣟
⣻⡾⣯⡿⣽⣻⣽⣟⣿⣻⣟⣿⣻⣽⡾⣯⣷⢿⣻⡾⣟⣿⣽⣟⣿⢾⣟⣿⣻⡷⣟⣷⢿⣽⣯⢿⣻⡾⣯⣿
⢷⡿⣯⣿⣻⣽⡷⣿⣽⢷⡿⣽⣯⣷⢿⣻⣽⣟⣯⣿⢿⣽⣾⣻⡾⣿⣽⣯⣷⢿⡿⣽⣟⣷⣟⣿⢯⣿⣻⣾
⣻⢿⣽⡾⣯⣷⣿⣻⡾⣟⣿⣻⣾⣽⢿⣻⣽⣯⢿⡾⣟⣷⣟⣷⢿⣻⣾⣳⣟⣿⣻⣯⣟⣷⢿⣽⢿⣽⢷⡿
⣟⣿⢷⣿⣻⣷⣻⣽⣟⣿⣽⢷⡿⣾⣟⣯⣷⢿⣻⣟⣿⣳⣿⣽⣟⣿⢾⣯⡿⣽⣷⣻⣽⣯⣿⣽⢿⣽⣟⡿"
    .to_string();

  if Args::parse().custom_ascii.is_none() {
    default_ascii
  } else {
    match std::fs::read_to_string(Args::parse().custom_ascii.unwrap()) {
      Ok(content) => content,
      _ => {
        println!("Could not find the specified file, falling back to default ascii art");
        default_ascii
      }
    }
  }
}
