# Cross-compilation

<https://rust-lang.github.io/rustup/cross-compilation.html>

e.g.) ubuntu

``` shell
sudo apt install gcc-aarch64-linux-gnu qemu-user
```

## Installing Toolchain

e.g.)

``` shell
rustup target add aarch64-unknown-linux-gnu
```


`~/.cargo/config.toml`

``` toml
[target.aarch64-unknown-linux-gnu]
linker="aarch64-linux-gnu-gcc"
```


Run using qemu

e.g.)

``` shell
qemu-aarch64 -L /usr/aarch64-linux-gnu ./hello
```
