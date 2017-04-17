# rand-bytes

A simple tool to generate cryptographically secure random bytes using a cryptographic pseudo-random number
generator.

Internally, `rand-bytes` uses [`ring`'s random number generator](https://briansmith.org/rustdoc/ring/rand/index.html).

## Usage

Install this with cargo:

```sh
cargo install rand-bytes
```

Or download one of the [releases](releases).

If your architecture/OS is not supported, you can simply build it yourself.

## Command line options

```text
Generate some random bytes

USAGE:
    rand-bytes [OPTIONS] <SIZE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <FILE>    Specify a file to output the bytes to, rather than stdout

ARGS:
    <SIZE>    Sets the number of bytes to generate random values for

```
