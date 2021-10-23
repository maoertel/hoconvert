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
