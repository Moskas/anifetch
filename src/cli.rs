use clap::Parser;
use clap_derive::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Username to query
  #[arg(short, long)]
  name: String,
  /// Use ascii art
  #[arg(short, long)]
  ascii: Option<bool>,
}

pub fn get_name() -> String {
  let args = Args::parse();
  args.name
}

pub fn use_ascii() -> bool {
  let args = Args::parse();
  match args.ascii {
    Some(true) => true,
    _ => false,
  }
}
