use clap::Parser;
use clap_derive::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Anilist username that you want to look up
  #[arg(short, long)]
  username: String,
  /// Disable ascii print
  #[arg(short, long, default_value_t = true, action = clap::ArgAction::SetFalse)]
  disable_ascii: bool,
  /// Load custom ascii from specified file
  #[arg(short, long, value_name = "CUSTOM_ASCII_PATH")]
  custom_ascii: Option<String>,
}

pub fn get_name() -> String {
  let args = Args::parse();
  args.username
}

pub fn use_ascii() -> bool {
  let args = Args::parse();
  args.disable_ascii
}

pub fn load_ascii() -> String {
  let args = Args::parse();
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

  if args.custom_ascii.is_none() {
    default_ascii
  } else {
    match std::fs::read_to_string(args.custom_ascii.unwrap()) {
      Ok(content) => content,
      _ => default_ascii,
    }
  }
}
