# {hocon:vert}

CLI Tool to convert `HOCON` into valid `JSON` or `YAML`.

Under normal circumstances this is mostly not needed because hocon configs are parsed 
within the application – case closed. But for example in conjunction with Kubernetes 
where `.conf` files can reside in `ConfigMaps` there was that need to extract information 
on command line from time to time. And what would be more comfortable than to use `jq` 
for this.   

## Usage

```bash
hoconvert [input | --file <path>] [--output (yaml|json)]
```

Either provide the hocon as first argument:

```bash 
hoconvert "foo = bar"
```

or provide it from `stdin`, 

```bash
echo "foo = bar" | hoconvert
``` 

which leads to the following output:

```json
{
  "foo": "bar"
}
```

You can also read the hocon from a file by providing the path:

```bash
hoconvert -f config.hocon
```

Here is an example of a real-life Kubernetes problem as stated above:

```bash
kubectl get cm <any ConfigMap> -o jsonpath='{.data.myHocon}' | jq -r | hoconvert | jq '.doWhatEverYouWant'
```

## Installation

### Install with homebrew

In case you use `brew` you can install `hoconvert` as follows:

```bash
brew tap maoertel/tap
brew install hoconvert
```

You can install from `brew` for the following architectures: `macOS/amd64`, `macOS/arm64` or `linux/amd64`.

### Download the binary

You can download a binary of the [latest release](https://github.com/maoertel/hoconvert/releases)
currently for `macOS/amd64`, `macOS/arm64` and `linux/amd64`.

### Install with cargo

In case you have `cargo` installed this is the easiest way to install `hoconvert` from 
[crates.io](https://crates.io/crates/hoconvert) in match to your underlying architecture:

```bash
cargo install hoconvert
```

### Build it yourself

Check this repo out, change into the project directory and run:
```bash
cargo build --release
```
