use std::str;

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

#[test]
fn given_empty_hocon_when_convert_then_empty_json() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();

  let assert = cmd.arg("{}").assert();

  let assert = assert.success();
  assert.stdout(predicate::str::contains("{}"));
}

#[test]
fn given_empty_string_when_convert_then_empty_json() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();

  let assert = cmd.arg("").assert();

  let assert = assert.success();
  assert.stdout(predicate::str::contains("{}"));
}

#[test]
fn given_simple_key_value_when_convert_then_simple_json_object() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();
  let command = cmd.arg("foo = bar").assert();

  let expected_json = r#"
        {
          "foo": "bar"
        }"#;
  let expected_json: Value = serde_json::from_str(expected_json).unwrap();

  let assert = command.success();
  let parsed_json = str::from_utf8(&assert.get_output().stdout).unwrap();
  let parsed_json: Value = serde_json::from_str(parsed_json).unwrap();

  assert_eq!(parsed_json, expected_json)
}

#[test]
fn given_a_hocon_object_when_convert_then_reflected_in_json_object() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();
  let command = cmd.arg("{ foo = { key = bar } }").assert();

  let test_json = r#"
        {
          "foo": {
            "key": "bar"
          }  
        }"#;
  let test_json: Value = serde_json::from_str(test_json).unwrap();

  let assert = command.success();
  let parsed_json = str::from_utf8(&assert.get_output().stdout).unwrap();
  let parsed_json: Value = serde_json::from_str(parsed_json).unwrap();

  assert_eq!(parsed_json, test_json)
}

#[test]
fn given_a_nested_hocon_object_when_convert_then_reflected_in_json_object() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();
  let command = cmd.arg("{ foo = { nested = { key = bar } } }").assert();

  let test_json = r#"
        {
          "foo": {
            "nested": {
              "key": "bar"
            }  
          }  
        }"#;
  let test_json: Value = serde_json::from_str(test_json).unwrap();

  let assert = command.success();
  let parsed_json = str::from_utf8(&assert.get_output().stdout).unwrap();
  let parsed_json: Value = serde_json::from_str(parsed_json).unwrap();

  assert_eq!(parsed_json, test_json)
}

#[test]
fn given_a_malformed_hocon_when_convert_then_error() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();
  let command = cmd.arg("{ foo = { nested = { key = bar ").assert();

  let assert = command.failure();
  assert.stderr(predicate::str::contains("Error: Parse"));
}

#[test]
fn given_a_key_without_value_when_convert_then_error() {
  let mut cmd = Command::cargo_bin("hoconvert").unwrap();
  let command = cmd.arg("{ foo = }").assert();

  let assert = command.failure();
  assert.stderr(predicate::str::contains("Error: Parse"));
}
