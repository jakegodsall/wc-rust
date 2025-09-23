# wc-rust

A basic clone of the Unix `wc` command built in Rust as a learning project for CLI development, basic I/O operations, and string methods.

## Usage

```bash
cargo run [options] [file]
```

**Options:**
- `-c` - count bytes
- `-w` - count words
- `-l` - count lines
- `-m` - count characters

Reads from stdin if no file is provided.