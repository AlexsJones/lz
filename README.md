# lz

A fast CLI tool to list the 5 most recently accessed files in a directory tree, sorted by last access time.

## Features
- Recursively scans a directory and its subdirectories
- Sorts files by last access time (most recently accessed first)
- Prints the 5 most recently accessed files with their access timestamps

## Installation

Add to your Cargo.toml or install via cargo:

```sh
cargo install --path .
```

## Usage

```sh
lz [OPTIONS] [--path <PATH>]
```

- `--path <PATH>`: The root directory to scan (defaults to current directory)

### Example

```sh
lz --path /var/log
```

Output:
```
/var/log/syslog  (2024-06-10 14:23:01)
/var/log/auth.log  (2024-06-10 13:55:12)
/var/log/kern.log  (2024-06-10 13:40:05)
/var/log/dpkg.log  (2024-06-10 12:10:44)
/var/log/faillog  (2024-06-10 11:59:33)
```

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).

## Contributing

Pull requests and issues are welcome! 