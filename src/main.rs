use crate::cli::Cli;
use crate::converter::Converter;

use clap::Parser;
use error::Result;
use std::io::{self, BufWriter, Read, Write};

mod cli;
mod converter;
mod error;

fn main() -> Result<()> {
  let Cli { string, file, output } = Cli::parse();

  // Use buffered stdout for better performance
  let stdout = io::stdout();
  let mut writer = BufWriter::new(stdout.lock());

  match (string, file) {
    (Some(string), _) => Converter::process_string(&string, output, &mut writer),
    (_, Some(file)) => Converter::process_file(&file, output, &mut writer),
    _ => {
      let mut input_buffer = String::new();
      io::stdin().read_to_string(&mut input_buffer)?;
      Converter::process_string(&input_buffer, output, &mut writer)
    }
  }?;

  // Ensure trailing newline for cleaner terminal output
  writeln!(writer)?;
  writer.flush()?;

  Ok(())
}
