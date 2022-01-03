# chronos - a command line speedrun timer

`chronos` is a lightweight and fast speedrun timer using livesplit-core
that runs in your terminal. With only about 900 lines of C, it is also 
really easy to modify for your needs.

### Usage

```
  Usage: ./chronos [options]

  Options:

    -V, --version                 output program version
    -h, --help                    output help information
    -c, --config <arg>            config file to use
    -l, --layout <arg>            layout file to use
    -s, --splits <arg>            split file to use
```

## Building

You will need the following dependencies:

* `make`
* latest stable `rustc`
* any c compiler with c99 support.

Run the following commands in your shell.

```
git clone --recursive https://github.com/darkrta/chronos
cd chronos
make
```
