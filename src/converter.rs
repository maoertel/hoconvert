use crate::cli::Output;
use crate::error::{Error, Result};

use hocon_rs::Config;
use serde_json::Value;
use std::io::ErrorKind;
use std::io::Write;

pub struct Converter;

impl Converter {
  pub(crate) fn process_string<W: Write>(hocon_string: &str, output: Output, writer: W) -> Result<()> {
    let value = Config::parse_str::<Value>(hocon_string, None)?;
    Converter::run(value, output, writer)
  }

  pub(crate) fn process_file<W: Write>(path: &str, output: Output, writer: W) -> Result<()> {
    let value = Config::parse_file::<Value>(path, None).map_err(|e| Converter::map_file_error(path, e))?;
    Converter::run(value, output, writer)
  }

  fn run<W: Write>(value: Value, output: Output, mut writer: W) -> Result<()> {
    match output {
      Output::Yaml => serde_yml::to_writer(writer, &value)?,
      Output::Json => serde_json::to_writer_pretty(writer, &value)?,
      Output::Toml => {
        let toml_str = toml::to_string_pretty(&value)?;
        writer.write_all(toml_str.as_bytes())?;
      }
    };

    Ok(())
  }

  /// Map a missing top-level file to a friendly message; pass other errors through.
  fn map_file_error(path: &str, error: hocon_rs::Error) -> Error {
    if let hocon_rs::Error::Io(io_error) = &error
      && io_error.kind() == ErrorKind::NotFound
    {
      return Error::PathNotFound(format!("Path '{path}' does not exist."));
    }

    Error::from(error)
  }
}

#[cfg(test)]
mod tests {
  use crate::cli::Output;
  use crate::converter::Converter;

  use serde_json::Value;
  use serde_json::json;

  fn to_value(input: &str) -> Value {
    let mut output = Vec::new();
    Converter::process_string(input, Output::Json, &mut output).unwrap();
    serde_json::from_slice(&output).unwrap()
  }

  #[test]
  fn empty_hocon_when_convert_empty_json() {
    // Given an empty HOCON input
    // When converting to JSON
    // Then the result is an empty object
    assert_eq!(to_value(""), json!({}));
  }

  #[test]
  fn simple_key_value_hocon_when_convert_reflected_in_json() {
    // Given a simple key/value HOCON
    // When converting to JSON
    // Then it reflects as a flat JSON object
    assert_eq!(to_value(r#"foo = bar"#), json!({ "foo": "bar" }));
  }

  #[test]
  fn hocon_object_when_convert_reflected_in_json() {
    // Given a HOCON object
    // When converting to JSON
    // Then the nesting is preserved
    assert_eq!(to_value(r#"{ foo = { key = bar } }"#), json!({ "foo": { "key": "bar" } }));
  }

  #[test]
  fn nested_hocon_object_when_convert_reflected_in_json() {
    // Given a deeply nested HOCON object
    // When converting to JSON
    // Then the full nesting is preserved
    assert_eq!(
      to_value(r#"{ foo = { nested = { key = bar } } }"#),
      json!({ "foo": { "nested": { "key": "bar" } } })
    );
  }

  #[test]
  fn malformed_hocon_when_convert_returns_parse_error() {
    // Given malformed HOCON
    // When converting
    // Then an error is returned
    let mut output = Vec::new();
    let result = Converter::process_string(r#"{ foo = { nested = { key = bar"#, Output::Json, &mut output);

    assert!(result.is_err());
  }

  #[test]
  fn streaming_output_works() {
    // Given a simple HOCON input
    // When converting to JSON into a writer
    // Then the streamed output contains the key and value
    let mut output = Vec::new();
    Converter::process_string(r#"foo = bar"#, Output::Json, &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.contains("\"foo\""));
    assert!(result.contains("\"bar\""));
  }

  #[test]
  fn toml_output_works() {
    // Given a simple HOCON input
    // When converting to TOML
    // Then the output contains the key and value
    let mut output = Vec::new();
    Converter::process_string(r#"foo = bar"#, Output::Toml, &mut output).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert!(result.contains("foo"));
    assert!(result.contains("bar"));
  }
}
