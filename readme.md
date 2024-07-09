# Mark OS
A really simple kernel built in rust. This is a learning project and is not intended to be used in any serious capacity.

Following along [Philipp Oppermann's](https://os.phil-opp.com/) blog series.

## Setup/Dependencies

```sh
# use the nightly build
rustup override set nightly

# We need to have a way to generate a bootable image, this will help
rustup component add llvm-tools-preview
```

We need a system to run the kernel, we will use [QEMU](https://www.qemu.org/download/#linux) for this, but you could use anything... 

on linux
```sh
sudo apt-get install qemu-system
```

Or if you're on mac
```sh
brew install qemu # if you're on mac
```

## Build

```sh
cargo build
cargo bootimage
```


## Run

cargo-bootimage runner` will load qemu and pass the kernel to it. 
to do this run

```sh
cargo run
```

Otherwise, if you want to invoke qemu manually, you can do so with the following command

```sh
qemu-system-x86_64 -drive format=raw,file=target/x86_64-mark_os/debug/bootimage-mark_os.bin
```
