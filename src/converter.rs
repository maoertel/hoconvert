use hocon::{Hocon, HoconLoader};
use serde_json::{Map, Number, Value};

use crate::error::Error;

pub struct Converter;

impl Converter {
  pub fn run(hocon_string: &str, yaml: bool) -> Result<String, Error> {
    let hocon = HoconLoader::new().load_str(hocon_string)?.hocon()?;
    let json = Converter::hocon_to_raw_json(hocon)?;

    let output = if yaml {
      serde_yaml::to_string(&json)?
    } else {
      serde_json::to_string_pretty(&json)?
    };

    Ok(output)
  }

  fn hocon_to_raw_json(hocon: Hocon) -> Result<Value, Error> {
    match hocon {
      Hocon::Boolean(b) => Ok(Value::Bool(b)),
      Hocon::Integer(i) => Ok(Value::Number(Number::from(i))),
      Hocon::Real(f) => Ok(Value::Number(Number::from_f64(f).unwrap())), // safe in this place, as we know that f is of type f64
      Hocon::String(s) => Ok(Value::String(s)),
      Hocon::Array(vec) => {
        let json_array: Result<Vec<Value>, Error> = vec.into_iter().map(Converter::hocon_to_raw_json).collect();
        Ok(Value::Array(json_array?))
      }
      Hocon::Hash(map) => {
        let json_object: Result<Map<String, Value>, Error> = map
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
}
