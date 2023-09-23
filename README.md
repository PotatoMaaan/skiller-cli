# skiller-cli

A cli to control the Skiller Pro+ keyboard on linux

## Getting started

### Installation

You can grab a binary from the releases page, or you can [build one yourself](##building)

### Usage

Here are some examples on how you might use this tool

```sh
skiller-cli color red   # set the color to red

skiller-cli -p 3 color blue   # set the color on profile 3 to blue

skiller-cli brightness static 5 white   # set the brightness to level 5

skiller-cli brightness cycle   # set the brightness to cycle through all colors

skiller-cli win-key off   # disable the windows key

skiller-cli -p 2 profile   # switch to profile 2

skiller-cli polling-rate 500   # set the polling rate to 500Hz
```

You can get more information amout each subcommand by passing the `--help` flag to the program like so:

```sh
> skiller-cli brightness static --help

A static color at the given brightness

Usage: skiller-cli brightness static <LEVEL> <COLOR>

Arguments:
  <LEVEL>
  <COLOR>  [possible values: red, green, blue, purple, cyan, yellow, white]
```

> **Note about the brightness subcommand:** You need to specify the color along with the brightness since the usb command to control the brightness also wants the color (no idea why they did it this way)

## Building

in order to build this program, you need to have a rust toolchain installed. You get it [here](https://rustup.rs/)

To build a ready-to-use binary just run:

```sh
cargo build --release
```
