# BUFR Dump

## Install

### MacOS (Apple Silicon)

``` shell
curl -o bufr -L https://github.com/castelao/BUFR/releases/latest/download/bufr-dump-aarch64-apple-darwin
chmod +x bufr
```

### MacOS (Intel)

``` shell
curl -o bufr -L https://github.com/castelao/BUFR/releases/latest/download/bufr-dump-x86_64-apple-darwin
chmod +x bufr
```

### Linux (arm)

``` shell
curl -o bufr -L https://github.com/castelao/BUFR/releases/latest/download/bufr-dump-arm-unknown-linux-gnueabihf
chmod +x bufr
```

### Linux (x86_64)

``` shell
curl -o bufr -L https://github.com/castelao/BUFR/releases/latest/download/bufr-dump-x86_64-unknown-linux-musl
chmod +x bufr
```

### Windows (x86_64)

``` shell
Invoke-WebRequest -Uri 'https://github.com/castelao/BUFR/releases/latest/download/bufr-dump-x86_64-pc-windows-msvc.exe' -OutFile bufr
```

### From source code

Install the Rust compiler if you don't already have it. Follow the
instructions at https://www.rust-lang.org/tools/install

Compile it by running: `cargo build --release -p bufr-dump`

## Options and customization

## Examples

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.68.0

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
