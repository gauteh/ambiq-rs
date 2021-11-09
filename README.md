# Ambiq-rs: HAL for Ambiq Apollo

This project consists of two main crates:

1.) [**ambiq-hal-sys**](ambiq-hal-sys/) is bindings to the Ambiq Suite SDK as modified by Sparkfun.

2.) **ambiq-hal** is a `embedded-hal` based Hardware Abstraction Layer (HAL) for
the Ambiq Apollo.

Eventually the board support crates will make it easier to determine which pins
(as labelled on the board) match the pads and peripherals on the Apollo MCU.

## Boards

* [Sparkfun Redboard Artemis](https://www.sparkfun.com/products/15444) - [boards/redboard](boards/redboard) | [boards/redboard-halc](boards/redboard-halc)
* [Sparkfun Redboard Artemis Nano](https://www.sparkfun.com/products/15443) - [boards/redboard-nano](boards/redboard-nano)

## MCU

* [Ambiq Apollo3](https://ambiq.com/apollo3-blue/) - [ambiq-hal/](ambiq-hal/)
