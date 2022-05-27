use std::str;

use assert_cmd::Command;
use predicates::prelude::*;
use serde_yaml::Value;

#[test]
fn given_empty_hocon_when_convert_then_empty_yaml() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();

  let assert = cmd.arg("{}").arg("--yaml").assert();

  let assert = assert.success();
  assert.stdout(predicate::str::contains("{}"));
}

#[test]
fn given_empty_string_when_convert_then_empty_yaml() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();

  let assert = cmd.arg("").arg("--yaml").assert();

  let assert = assert.success();
  assert.stdout(predicate::str::contains("{}"));
}

#[test]
fn given_simple_key_value_when_convert_then_simple_yaml_object() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();
  let command = cmd.arg("foo = bar").arg("--yaml").assert();

  let expected_yaml = r#"foo: "bar""#;
  let expected_yaml: Value = serde_yaml::from_str(expected_yaml).unwrap();

  let assert = command.success();
  let parsed_yaml = str::from_utf8(&assert.get_output().stdout).unwrap();
  let parsed_yaml: Value = serde_yaml::from_str(parsed_yaml).unwrap();

  assert_eq!(parsed_yaml, expected_yaml)
}

#[test]
fn given_a_hocon_object_when_convert_then_reflected_in_yaml_object() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();
  let command = cmd.arg("{ foo = { key = bar } }").assert();

  let test_yaml = r#"
        foo: 
          key: "bar"
        "#;
  let test_yaml: Value = serde_yaml::from_str(test_yaml).unwrap();

  let assert = command.success();
  let parsed_yaml = str::from_utf8(&assert.get_output().stdout).unwrap();
  let parsed_yaml: Value = serde_yaml::from_str(parsed_yaml).unwrap();

  assert_eq!(parsed_yaml, test_yaml)
}

#[test]
fn given_a_nested_hocon_object_when_convert_then_reflected_in_yaml_object() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();
  let command = cmd.arg("{ foo = { nested = { key = bar } } }").arg("--yaml").assert();

  let test_yaml = r#"
        foo:
          nested:
            key: "bar"
        "#;
  let test_yaml: Value = serde_yaml::from_str(test_yaml).unwrap();

  let assert = command.success();
  let parsed_yaml = str::from_utf8(&assert.get_output().stdout).unwrap();
  let parsed_yaml: Value = serde_yaml::from_str(parsed_yaml).unwrap();

  assert_eq!(parsed_yaml, test_yaml)
}

#[test]
fn given_a_malformed_hocon_when_convert_then_error() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();
  let command = cmd.arg("{ foo = { nested = { key = bar ").arg("--yaml").assert();

  let assert = command.failure();
  assert.stderr(predicate::str::contains("Error: Hocon(Parse)"));
}

#[test]
fn given_a_key_without_value_when_convert_then_error() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();
  let command = cmd.arg("{ foo = }").arg("--yaml").assert();

  let assert = command.failure();
  assert.stderr(predicate::str::contains("Error: Hocon(Parse)"));
}
