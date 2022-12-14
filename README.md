<div align="center">
    <h1>file-rs</h1>
    <p>a tool for determining file types, an alternative to file</p>
</div>

## whats done

- [x] determining file extension
- [x] determining file type
- [x] determining file's mime type

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
- on an old core2 laptop and even on old computers file-rs performs a lot better than file
- in this repo(current dir)

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
