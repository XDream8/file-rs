<div align="center">
    <h1>file-rs</h1>
    <p>a tool for determining file types, an alternative to file</p>
</div>

[![file-rs](https://asciinema.org/a/787432.svg)](https://asciinema.org/a/787432)

## whats done

- [x] determining file extension
- [x] determining file type
- [x] determining file's mime type

## install using **cargo**
```sh
$ cargo install file-rs
```

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
- ‘file-rs‘ version: , used mime-type library: infer
- ‘file‘ version: 5.46
- runs: 100
[![file-rs vs file](https://asciinema.org/a/787434.svg)](https://asciinema.org/a/787434)
