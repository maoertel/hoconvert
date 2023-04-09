use crate::cli::Cli;
use crate::converter::Converter;

use clap::Parser;
use error::Result;
use std::io::Read;

mod cli;
mod converter;
mod error;

fn main() -> Result<()> {
  let Cli { string, file, output } = Cli::parse();

  let result = match (string, file) {
    (Some(string), _) => Converter::process_string(&string, output),
    (_, Some(file)) => Converter::process_file(&file, output),
    _ => {
      let mut input_buffer = String::new();
      std::io::stdin().read_to_string(&mut input_buffer)?;
      Converter::process_string(&input_buffer, output)
    }
  };

  result.map(|output| println!("{output}"))
}
