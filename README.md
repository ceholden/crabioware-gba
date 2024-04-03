# :crab: crabioware-gba :crab:

> Pasé la Gold, en el GameBoy

CrabioWare GBA is technically a video game for the GameBoy Advanced (GBA) that
runs micro-games, similar to games starring the inverse of a popular jumpman.

This project is built using the [agbrs/agb](https://github.com/agbrs/agb) crate
which abstracts low level hardware interactions, exports useful code, and
enables great developer experience (Aseprite imports, ROM building, mGBA testing, etc).
The [rust-console/gba](https://github.com/rust-console/gba) crate is also very
useful for getting a better understanding of the hardware.

Goals of this project in rough order,

* Learn Rust and practice writing idiomatically
* Implement common design patterns in Rust
* Learn more about the GBA
* Learn more about game development and algorithms
* Implement classic games with Rust memes
* ... many other goals
* Make good games that are fun to play


## Setup

Some commands:

Run your game,
```shell
$ just run-game
# opens mGBA-Qt and runs your game
```

Run tests,
```shell
$ just test
# runs tests... note that mGBA-Qt will pop up
```

Build a ROM that can be run on real hardware,
```shell
$ just build-roms
# create a ROM like, `crabioware/crabioware_[datetime].gba`
```

### Host

You will need,

* `rustup`
    * To add components and nightly
* Rust source for compiling
    * `rustup component add rust-src`
* `just` for running commands
    * `cargo install just`
    * https://github.com/casey/just
* Sprites are created using aseprite
    * https://github.com/aseprite/aseprite
    * https://www.aseprite.org/

### Docker

```bash
./tools/build
```

to build the container. You can run a shell in it using the helper script,

```bash
./tools/shell
```
