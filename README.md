# rstatic
Static file serve by Rust.

### Build

```sh
git clone https://github.com/iorust/rstatic.git && cd rstatic && cargo build --release
```

Then exec file `rstatic/target/release/rstatic` created.

if openssl-sys build error:
```
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
```

### Run

```sh
target/release/rstatic
```

More help:
```sh
target/release/rstatic --help
```
