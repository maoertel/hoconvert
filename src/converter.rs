use crate::cli::Output;
use crate::error::{Error, Result};

use hocon::{Hocon, HoconLoader};
use serde_json::{Map, Number, Value};
use std::io::Write;

pub struct Converter;

impl Converter {
  pub(crate) fn process_string<W: Write>(hocon_string: &str, output: Output, writer: W) -> Result<()> {
    let hocon = HoconLoader::new().load_str(hocon_string)?.hocon()?;
    Converter::run(hocon, output, writer)
  }

  pub(crate) fn process_file<W: Write>(path: &str, output: Output, writer: W) -> Result<()> {
    let hocon = HoconLoader::new()
      .load_file(path)
      .map_err(|e| match e {
        hocon::Error::Include { path } => Error::PathNotFound(format!("Path '{path}' does not exist.")),
        other => Error::from(other),
      })?
      .hocon()?;
    Converter::run(hocon, output, writer)
  }

  fn run<W: Write>(hocon: Hocon, output: Output, writer: W) -> Result<()> {
    let json = Converter::hocon_to_raw_json(hocon)?;

    match output {
      Output::Yaml => serde_yml::to_writer(writer, &json)?,
      Output::Json => serde_json::to_writer_pretty(writer, &json)?,
    };

    Ok(())
  }

  fn hocon_to_raw_json(hocon: Hocon) -> Result<Value> {
    match hocon {
      Hocon::Boolean(b) => Ok(Value::Bool(b)),
      Hocon::Integer(i) => Ok(Value::Number(Number::from(i))),
      Hocon::Real(f) => {
        // Handle NaN and Infinity which can't be represented in JSON
        Number::from_f64(f).map(Value::Number).ok_or(Error::InvalidFloat(f))
      }
      Hocon::String(s) => Ok(Value::String(s)),
      Hocon::Array(vec) => {
        let json_array: Result<Vec<Value>> = vec.into_iter().map(Converter::hocon_to_raw_json).collect();
        Ok(Value::Array(json_array?))
      }
      Hocon::Hash(map) => {
        let json_object: Result<Map<String, Value>> = map
          .into_iter()
          .map(|(k, v)| Ok((k, Converter::hocon_to_raw_json(v)?)))
          .collect();

        Ok(Value::Object(json_object?))
      }
      Hocon::Null => Ok(Value::Null),
      Hocon::BadValue(bad_value) => Err(Error::from(bad_value)),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::cli::Output;
  use crate::converter::Converter;
  use hocon::HoconLoader;

  #[test]
  fn empty_hocon_when_convert_empty_json() {
    let hocon = "";
    let hocon = HoconLoader::new().load_str(hocon).unwrap().hocon().unwrap();

    let json = Converter::hocon_to_raw_json(hocon).unwrap();

    let test_json = "{}";
    let parsed_json = serde_json::to_string(&json).unwrap();
    assert_eq!(test_json, parsed_json)
  }

  #[test]
  fn simple_key_value_hocon_when_convert_reflected_in_json() {
    let hocon = r#"foo = bar"#;
    let hocon = HoconLoader::new().load_str(hocon).unwrap().hocon().unwrap();

    let json = Converter::hocon_to_raw_json(hocon).unwrap();

    let test_json = r#"{"foo":"bar"}"#;
    let parsed_json = serde_json::to_string(&json).unwrap();
    assert_eq!(test_json, parsed_json)
  }

  #[test]
  fn hocon_object_when_convert_reflected_in_json() {
    let hocon = r#"{ foo = { key = bar } }"#;
    let hocon = HoconLoader::new().load_str(hocon).unwrap().hocon().unwrap();

    let json = Converter::hocon_to_raw_json(hocon).unwrap();

    let test_json = r#"{"foo":{"key":"bar"}}"#;
    let parsed_json = serde_json::to_string(&json).unwrap();
    assert_eq!(test_json, parsed_json)
  }

  #[test]
  fn nested_hocon_object_when_convert_reflected_in_json() {
    let hocon = r#"{ foo = { nested = { key = bar } } }"#;
    let hocon = HoconLoader::new().load_str(hocon).unwrap().hocon().unwrap();

    let json = Converter::hocon_to_raw_json(hocon).unwrap();

    let test_json = r#"{"foo":{"nested":{"key":"bar"}}}"#;
    let parsed_json = serde_json::to_string(&json).unwrap();
    assert_eq!(test_json, parsed_json)
  }

  #[test]
  fn malformed_hocon_when_convert_returns_parse_error() {
    let hocon = r#"{ foo = { nested = { key = bar"#;
    let parse_error = HoconLoader::new().load_str(hocon).expect_err("This should fail");

    assert_eq!(parse_error, hocon::Error::Parse)
  }

  #[test]
  fn streaming_output_works() {
    let mut output = Vec::new();
    Converter::process_string(r#"foo = bar"#, Output::Json, &mut output).unwrap();
    let result = String::from_utf8(output).unwrap();
    assert!(result.contains("\"foo\""));
    assert!(result.contains("\"bar\""));
  }
}
