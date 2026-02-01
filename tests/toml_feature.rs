use std::str;

use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use toml::Value;

#[test]
fn given_empty_hocon_when_convert_then_empty_toml() {
  let mut cmd = cargo_bin_cmd!("hoconvert");

  let assert = cmd.arg("{}").arg("--output").arg("toml").assert();

  let assert = assert.success();
  // Empty TOML is just empty string or whitespace
  assert.stdout(predicate::str::is_empty().or(predicate::str::is_match(r"^\s*$").unwrap()));
}

#[test]
fn given_simple_key_value_when_convert_then_simple_toml() {
  let mut cmd = cargo_bin_cmd!("hoconvert");
  let command = cmd.arg("foo = bar").arg("--output").arg("toml").assert();

  let expected_toml = r#"foo = "bar""#;
  let expected_toml: Value = toml::from_str(expected_toml).unwrap();

  let assert = command.success();
  let parsed_toml = str::from_utf8(&assert.get_output().stdout).unwrap();
  let parsed_toml: Value = toml::from_str(parsed_toml).unwrap();

  assert_eq!(parsed_toml, expected_toml)
}

#[test]
fn given_a_hocon_object_when_convert_then_reflected_in_toml() {
  let mut cmd = cargo_bin_cmd!("hoconvert");
  let command = cmd.arg("{ foo = { key = bar } }").arg("--output").arg("toml").assert();

  let test_toml = r#"
        [foo]
        key = "bar"
        "#;
  let test_toml: Value = toml::from_str(test_toml).unwrap();

  let assert = command.success();
  let parsed_toml = str::from_utf8(&assert.get_output().stdout).unwrap();
  let parsed_toml: Value = toml::from_str(parsed_toml).unwrap();

  assert_eq!(parsed_toml, test_toml)
}

#[test]
fn given_a_nested_hocon_object_when_convert_then_reflected_in_toml() {
  let mut cmd = cargo_bin_cmd!("hoconvert");
  let command = cmd
    .arg("{ foo = { nested = { key = bar } } }")
    .arg("--output")
    .arg("toml")
    .assert();

  let test_toml = r#"
        [foo.nested]
        key = "bar"
        "#;
  let test_toml: Value = toml::from_str(test_toml).unwrap();

  let assert = command.success();
  let parsed_toml = str::from_utf8(&assert.get_output().stdout).unwrap();
  let parsed_toml: Value = toml::from_str(parsed_toml).unwrap();

  assert_eq!(parsed_toml, test_toml)
}

#[test]
fn given_a_malformed_hocon_when_convert_then_error() {
  let mut cmd = cargo_bin_cmd!("hoconvert");
  let command = cmd
    .arg("{ foo = { nested = { key = bar ")
    .arg("--output")
    .arg("toml")
    .assert();

  let assert = command.failure();
  assert.stderr(predicate::str::contains("Error: Hocon(Parse)"));
}
