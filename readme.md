# darksplit - a command line speedrun timer

`darksplit` is a lightweight and fast speedrun timer using livesplit-core
that runs in your terminal. With about 600 lines of C you are able to have a
suprisingly usable timer that is also extremely lightweight.

### Usage

```
  Usage: darksplit [options]

  Options:

    -V, --version                 output program version
    -h, --help                    output help information
    -l, --layout <arg>            layout file to use
    -s, --splits <arg>            split file to use
```

## Building

You will need the following dependencies:

* `libncursesw`
* `make`
* latest stable `rustc`
* any c compiler with c99 support.

Get the submodules, edit the Makefile, then run `make`

# Configuration:

You are intended to configure darksplit by modifying its code. It is pointless
to make binary packages of it for this reason. Nevertheless, most of the stuff 
that you'd want to modify is in `config.c`.

# Licensing Info

`darksplit` is licensed under either of these two licences at your choosing:

- GPL 2.0
- MIT

The text for these licenses can be found in `./LICENSE-GPL` or `./LICENSE-MIT`
respectively.

This license does not apply to any file under `./deps/`.
