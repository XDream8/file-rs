<div align="center">
    <h1>file-rs</h1>
    <p>a tool for determining file types, an alternative to file</p>
</div>

## whats done

- [x] determining file extension
- [x] determining file type
- [x] determining file's mime type

## building from git source
you have 2 options for mime type detection library. one of them must be enabled:
- infer(default. smaller but may perform slower sometimes)
- mime_guess

```sh
$ git clone https://codeberg.org/XDream8/file-rs
$ cd file-rs
$ cargo build --profile optimized --no-default-features --features mime_guess
$ ./target/optimized/file-rs
```

## usage

```sh
$ file-rs -h
$ file-rs <files> <args>
```

- example:

```sh
$ file-rs Cargo.toml
```

## benchmarks
- ‘file-rs‘ used mime-type library: infer
- ‘file‘ version: 5.44
- runs: 100
![Benchmark1](https://codeberg.org/XDream8/file-rs/raw/branch/main/bench1.png)
![Benchmark2](https://codeberg.org/XDream8/file-rs/raw/branch/main/bench2.png)
![Benchmark3](https://codeberg.org/XDream8/file-rs/raw/branch/main/bench3.png)
