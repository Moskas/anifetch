use clap::Parser;
use clap_derive::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Username to query
  #[arg(short, long)]
  name: String,
}

pub fn get_name() -> String {
  let args = Args::parse();
  args.name
}
