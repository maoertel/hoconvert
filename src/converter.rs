use hocon::{Error, Hocon, HoconLoader};
use serde_json::{Number, Value};

pub struct Converter;

impl Converter {
  pub fn run(hocon_string: String, yaml: bool) -> Result<String, Error> {
    let hocon = HoconLoader::new().load_str(&hocon_string)?.hocon()?;
    let json: Option<Value> = Converter::hocon_to_json(hocon);

    let output = if yaml {
      serde_yaml::to_string(&json).map_err(|e| Error::Deserialization { message: e.to_string() })?
    } else {
      serde_json::to_string_pretty(&json).map_err(|e| Error::Deserialization { message: e.to_string() })?
    };

    Ok(output)
  }

  /* Function taken from 'Hocon' crate authored by François Mockers: https://docs.rs/hocon */
  fn hocon_to_json(hocon: Hocon) -> Option<Value> {
    match hocon {
      Hocon::Boolean(b) => Some(Value::Bool(b)),
      Hocon::Integer(i) => Some(Value::Number(Number::from(i))),
      Hocon::Real(f) => Some(Value::Number(Number::from_f64(f).unwrap_or(Number::from(0)))),
      Hocon::String(s) => Some(Value::String(s)),
      Hocon::Array(vec) => Some(Value::Array(
        vec.into_iter().map(Converter::hocon_to_json).filter_map(|i| i).collect(),
      )),
      Hocon::Hash(map) => Some(Value::Object(
        map
          .into_iter()
          .map(|(k, v)| (k, Converter::hocon_to_json(v)))
          .filter_map(|(k, v)| v.map(|v| (k, v)))
          .collect(),
      )),
      Hocon::Null => Some(Value::Null),
      Hocon::BadValue(_) => None,
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
    let hocon = HoconLoader::new().load_str(&hocon).unwrap().hocon().unwrap();

    let json = Converter::hocon_to_json(hocon);

    let test_json = "{}";
    let parsed_json = serde_json::to_string(&json).unwrap();
    assert_eq!(test_json, parsed_json)
  }

  #[test]
  fn simple_key_value_hocon_when_convert_reflected_in_json() {
    let hocon = r#"foo = bar"#;
    let hocon = HoconLoader::new().load_str(&hocon).unwrap().hocon().unwrap();

    let json = Converter::hocon_to_json(hocon);

    let test_json = r#"{"foo":"bar"}"#;
    let parsed_json = serde_json::to_string(&json).unwrap();
    assert_eq!(test_json, parsed_json)
  }

  #[test]
  fn hocon_object_when_convert_reflected_in_json() {
    let hocon = r#"{ foo = { key = bar } }"#;
    let hocon = HoconLoader::new().load_str(&hocon).unwrap().hocon().unwrap();

    let json = Converter::hocon_to_json(hocon);

    let test_json = r#"{"foo":{"key":"bar"}}"#;
    let parsed_json = serde_json::to_string(&json).unwrap();
    assert_eq!(test_json, parsed_json)
  }

  #[test]
  fn nested_hocon_object_when_convert_reflected_in_json() {
    let hocon = r#"{ foo = { nested = { key = bar } } }"#;
    let hocon = HoconLoader::new().load_str(&hocon).unwrap().hocon().unwrap();

    let json = Converter::hocon_to_json(hocon);

    let test_json = r#"{"foo":{"nested":{"key":"bar"}}}"#;
    let parsed_json = serde_json::to_string(&json).unwrap();
    assert_eq!(test_json, parsed_json)
  }

  #[test]
  fn malformed_hocon_when_convert_returns_parse_error() {
    let hocon = r#"{ foo = { nested = { key = bar"#;
    let parse_error = HoconLoader::new().load_str(&hocon).expect_err("This should fail");

    assert_eq!(parse_error, hocon::Error::Parse)
  }
}
