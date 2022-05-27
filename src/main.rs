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
  hocon: Option<String>,
  /// Optional flag. If you want the output to be YAML.
  #[clap(long, short)]
  yaml: bool,
}

fn main() -> Result<(), Error> {
  let opts: Opts = Opts::parse();

  let input = match opts.hocon {
    Some(input) => input,
    None => {
      let mut input_buffer = String::new();
      std::io::stdin().read_to_string(&mut input_buffer)?;
      input_buffer
    }
  };

  Converter::run(&input, opts.yaml).map(|output| println!("{}", output))
}
