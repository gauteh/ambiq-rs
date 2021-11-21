# Ambiq-rs: HAL for Ambiq Apollo

This project consists of two main crates:

1.) [**ambiq-hal-sys**](ambiq-hal-sys/) is bindings to the Ambiq Suite SDK as modified by Sparkfun.

2.) [**ambiq-hal**](ambiq-hal/) is a `embedded-hal` based Hardware Abstraction Layer (HAL) for
the Ambiq Apollo.

Eventually the board support crates will make it easier to determine which pins
(as labelled on the board) match the pads and peripherals on the Apollo MCU.

## Getting started

Take a look at the example in [quickstart](quickstart/) for how to get started.
Remeber to select the correct board.

## Boards

The board crates are not very useful yet.

* [Sparkfun Redboard Artemis](https://www.sparkfun.com/products/15444) - [boards/redboard](boards/redboard) | [boards/redboard-halc](boards/redboard-halc)
* [Sparkfun Redboard Artemis Nano](https://www.sparkfun.com/products/15443) - [boards/redboard-nano](boards/redboard-nano)

## MCUs

* [Ambiq Apollo3](https://ambiq.com/apollo3-blue/) - [ambiq-hal/](ambiq-hal/)

## Interrupts and `pre_init`

The Sparkfun bootloader does not update the pointer to the interrupt vectors
(this should be done by the C runtime). We therefore set the correct address in
`cortex_m_rt::pre_init`, this means that you can't use `pre_init` for your code.

