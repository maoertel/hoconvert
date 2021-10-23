# hoconvert

Tool to convert `HOCON` into valid `JSON` or `YAML` written in `Rust`.

## Usage

`hoconvert [input] [--yaml]`

Either provide the hocon as first argument:

`hoconvert "foo = bar"` 

or provide it from `stdin`, 

`echo "foo = bar" | hoconvert` which leads to the following output:

```json
{
  "foo": "bar"
}
```

Example of a real-life problem:

`kubectl get cm <any ConfigMap> -o jsonpath='{.data.myHocon} | jq -r | hoconvert`
