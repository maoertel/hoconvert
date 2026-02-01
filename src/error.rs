use std::fmt;
use std::fmt::{Debug, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Hocon(hocon::Error),
  Json(serde_json::Error),
  Yaml(serde_yml::Error),
  Toml(toml::ser::Error),
  IO(std::io::Error),
  PathNotFound(String),
  InvalidFloat(f64),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Error::Hocon(e) => std::fmt::Display::fmt(e, f),
      Error::Json(e) => std::fmt::Display::fmt(e, f),
      Error::Yaml(e) => std::fmt::Display::fmt(e, f),
      Error::Toml(e) => std::fmt::Display::fmt(e, f),
      Error::IO(e) => std::fmt::Display::fmt(e, f),
      Error::PathNotFound(e) => std::fmt::Display::fmt(e, f),
      Error::InvalidFloat(val) => write!(
        f,
        "Invalid float value: {val} (NaN or Infinity cannot be represented in JSON/TOML)"
      ),
    }
  }
}

impl From<hocon::Error> for Error {
  fn from(hocon_error: hocon::Error) -> Self {
    Error::Hocon(hocon_error)
  }
}

impl From<serde_json::Error> for Error {
  fn from(json_error: serde_json::Error) -> Self {
    Error::Json(json_error)
  }
}

impl From<serde_yml::Error> for Error {
  fn from(yaml_error: serde_yml::Error) -> Self {
    Error::Yaml(yaml_error)
  }
}

impl From<toml::ser::Error> for Error {
  fn from(toml_error: toml::ser::Error) -> Self {
    Error::Toml(toml_error)
  }
}

impl From<std::io::Error> for Error {
  fn from(io_error: std::io::Error) -> Self {
    Error::IO(io_error)
  }
}
