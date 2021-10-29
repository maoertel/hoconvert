# hoconvert

CLI Tool to convert `HOCON` into valid `JSON` or `YAML`.

Under normal circumstances this is mostly not needed because hocon configs are parsed 
within the application – case closed. But in conjunction with Kubernetes where `.conf` 
files can reside in `ConfigMaps` there was that need to extract information on command 
line from time to time. And what would be more comfortable than to use `jq` for this.   

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

Here is an example of a real-life Kubernetes problem as stated above:

`kubectl get cm <any ConfigMap> -o jsonpath='{.data.myHocon}' | jq -r | hoconvert | jq '.doWhatEverYouWant'`
