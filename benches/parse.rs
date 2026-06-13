//! Benchmarks for hoconvert's parse-and-convert pipeline.
//!
//! These mirror what `Converter` does: parse HOCON into a `serde_json::Value`
//! with `hocon-rs`, then serialize to JSON, YAML, or TOML.

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::criterion_group;
use criterion::criterion_main;

use hocon_rs::Config;
use serde_json::Value;

/// A representative mid-size config exercising nesting, arrays of objects,
/// substitutions, and value concatenation. Kept TOML-compatible (no nulls).
const CONFIG: &str = r#"
app {
  name = "my-service"
  version = "1.2.3"
  debug = false
  workers = 8
  timeout-ms = 30000
}
server {
  host = "0.0.0.0"
  port = 8080
}
database {
  host = "localhost"
  port = 5432
  pool { min = 2, max = 16 }
  replicas = [
    { host = "r1", port = 5433 },
    { host = "r2", port = 5434 },
    { host = "r3", port = 5435 }
  ]
}
logging {
  level = "info"
  appenders = [console, file, syslog]
}
feature-flags {
  a = true
  b = false
  c = ${app.debug}
}
"#;

fn parse(input: &str) -> Value {
  Config::parse_str::<Value>(input, None).expect("valid HOCON")
}

fn bench_parse(c: &mut Criterion) {
  let mut group = c.benchmark_group("parse");
  group.throughput(Throughput::Bytes(CONFIG.len() as u64));
  group.bench_function("config", |b| b.iter(|| parse(CONFIG)));

  for (name, input) in [
    ("kv", "foo = bar"),
    ("nested", "{ a = { b = { c = { d = 1 } } } }"),
    ("array", "a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"),
    ("substitution", "a = 1\nb = ${a}\nc = ${b}\nd = ${c}"),
  ] {
    group.bench_with_input(BenchmarkId::new("small", name), input, |b, i| b.iter(|| parse(i)));
  }
  group.finish();
}

fn bench_convert(c: &mut Criterion) {
  let value = parse(CONFIG);

  let mut group = c.benchmark_group("convert");
  group.bench_function("json", |b| b.iter(|| serde_json::to_vec_pretty(&value).expect("json")));
  group.bench_function("yaml", |b| b.iter(|| serde_yml::to_string(&value).expect("yaml")));
  group.bench_function("toml", |b| b.iter(|| toml::to_string_pretty(&value).expect("toml")));
  group.finish();
}

criterion_group!(benches, bench_parse, bench_convert);
criterion_main!(benches);
