# Mark OS
A really simple kernel built in rust. This is a learning project and is not intended to be used in any serious capacity.

Following along [Philipp Oppermann](https://os.phil-opp.com/) blog series.

## Setup

```sh
# install the rustup toolchain
rustup target add thumbv7em-none-eabihf

# use the nightly build
rustup override set nightly
```

## Build

```sh
cargo build --target thumbv7em-none-eabihf
``
