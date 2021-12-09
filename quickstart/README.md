# Quickstart with Rust on the Ambiq Apollo / Sparkfun Redboard Artemis

This quickstart is adapted to the Sparkfun Redboard Artemis. The pins for e.g.
LED or I2C ports differ between boards.

## Installing the required toolchains and targets

You essentially have to follow the [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart),
start with adding the target for our MCU:

```
$ rustup target add thumbv7em-none-eabihf
```

## Build and upload using the Sparkfun bootloader

```
$ cargo run && tail /dev/ttyACM0
```

## Using probe-run

If you have a debugger, and have [this patch](https://github.com/probe-rs/probe-rs/pull/855) applied to probe-rs (as used by
probe-run), you can set the runner to be `probe-run` in `.cargo/config.toml`:

```
$ cargo run --bin rtt
```

## New projects

Remember to include `build.rs`, `memory.x` and the `.cargo` subfolder in your
project.

