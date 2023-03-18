use std::io::Read;

use clap::Parser;
use error::Error;

use crate::converter::Converter;

mod converter;
mod error;

/// Converts a hocon into JSON (default) or YAML ('--yaml').
#[derive(Parser)]
#[clap(version = "0.1.3", author = "Mathias Oertel <mathias.oertel@pm.me>")]
struct Opts {
  /// Has to be a valid HOCON. Provided either as first argument or from stdin.
  #[clap(conflicts_with = "file")]
  hocon: Option<String>,
  /// HOCON file to process.
  #[clap(long, short, conflicts_with = "hocon")]
  file: Option<String>,
  /// Optional flag. If you want the output to be YAML.
  #[clap(long, short)]
  yaml: bool,
}

fn main() -> Result<(), Error> {
  let opts: Opts = Opts::parse();

  let result = match (opts.hocon, opts.file) {
    (Some(hocon), _) => Converter::process_string(&hocon, opts.yaml),
    (_, Some(file)) => Converter::process_file(&file, opts.yaml),
    (None, None) => {
      let mut input_buffer = String::new();
      std::io::stdin().read_to_string(&mut input_buffer)?;
      Converter::process_string(&input_buffer, opts.yaml)
    }
  };

  result.map(|output| println!("{}", output))
}
