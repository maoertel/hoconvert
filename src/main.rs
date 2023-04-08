use std::io::Read;

use clap::Parser;
use error::Error;

use crate::converter::Converter;

mod converter;
mod error;

#[derive(clap::ValueEnum, Clone)]
#[clap(rename_all = "lower")]
enum Output {
  Yaml,
  Json,
}

/// Converts a hocon into JSON (default) or YAML.
#[derive(Parser)]
#[clap(version = "0.1.3", author = "Mathias Oertel <mathias.oertel@pm.me>")]
struct Cli {
  /// Has to be a valid HOCON representation. Provided either as first argument or from stdin.
  #[clap(conflicts_with = "file")]
  string: Option<String>,

  /// File path to load the hocon for.
  #[clap(long, short, conflicts_with = "string")]
  file: Option<String>,

  /// Option to speciy the output format.
  #[clap(value_enum)]
  #[clap(long, short, default_value = "json")]
  output: Output,
}

fn main() -> Result<(), Error> {
  let Cli { string, file, output } = Cli::parse();

  let result = match (string, file) {
    (Some(string), _) => Converter::process_string(&string, output),
    (_, Some(file)) => Converter::process_file(&file, output),
    (None, None) => {
      let mut input_buffer = String::new();
      std::io::stdin().read_to_string(&mut input_buffer)?;
      Converter::process_string(&input_buffer, output)
    }
  };

  result.map(|output| println!("{output}"))
}
