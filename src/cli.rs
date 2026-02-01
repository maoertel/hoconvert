use clap::Parser;

#[derive(clap::ValueEnum, Clone)]
#[clap(rename_all = "lower")]
pub(crate) enum Output {
  Yaml,
  Json,
  Toml,
}

/// Converts a hocon into JSON (default), YAML, or TOML.
#[derive(Parser)]
#[clap(version = "1.0.0", author = "Mathias Oertel <mathias.oertel@pm.me>")]
pub(crate) struct Cli {
  /// Has to be a valid HOCON representation. Provided either as first argument or from stdin.
  #[clap(conflicts_with = "file")]
  pub(crate) string: Option<String>,

  /// File path to load the hocon from.
  #[clap(long, short, conflicts_with = "string")]
  pub(crate) file: Option<String>,

  /// Option to speciy the output format.
  #[clap(value_enum)]
  #[clap(long, short, default_value = "json")]
  pub(crate) output: Output,
}
