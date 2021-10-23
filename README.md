# hoconvert

Tool to convert `HOCON` into valid `JSON` or `YAML` written in `Rust`.

## Usage

`hoconvert [input] [--yaml]`

Either provide the hocon as first argument:

`hoconvert "foo = bar"` which leads to the following output:

```json
{
  "foo": "bar"
}
```

or provide it from `stdin`, e.g. a real-life problem:

`kubectl get cm <any ConfigMap> -o jsonpath='{.data.myHocon} | hoconvert`
