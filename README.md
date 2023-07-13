<div align="center">
    <h1>file-rs</h1>
    <p>a tool for determining file types, an alternative to file</p>
</div>

## whats done

- [x] determining file extension
- [x] determining file type
- [x] determining file's mime type

## building from git source
```sh
$ git clone https://codeberg.org/XDream8/file-rs
$ cd file-rs
$ cargo build --profile optimized
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

my benchmarks shows that file-rs is almost %98 faster than file.

these benchmarks were done:

- to calculate the performance difference between file-rs and file
- in this repo(current dir)
- even on old computers file-rs performs a lot better than file

### 1

- file:

```sh
$ ./benchmark.sh file
94.274010965
```

- file-rs:

```sh
$ ./benchmark.sh file-rs
2.444943173
```

### 2

- file:

```sh
$ ./benchmark.sh file
97.173416678
```

- file-rs:

```sh
$ ./benchmark.sh file-rs
2.461263894
```
