# rusty-turing-machine

[![Crates.io](https://img.shields.io/crates/v/linear_clock.svg)](https://crates.io/crates/linear_clock)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/functional-tim/rusty-turing-machine/blob/main/LICENSE-MIT)
[![dependency status](https://deps.rs/repo/github/functional-tim/linear_clock/status.svg)](https://deps.rs/repo/github/functional-tim/linear_clock)

-----------------------------------------------

## Usage

You can either let it run continously in the console or let it run once to get a string. The later can be used for example in conky. With the Option `-d` you can also get the date of the day.

```
linear_clock 0.1.1
A program to show a linear clock with date.
Sources and licenses are found here: https://github.com/functional-tim/linear_clock

USAGE:
    linear_clock [FLAGS]

FLAGS:
    -c, --continous    Runs the program continously
    -d, --date         Prints also the date
    -h, --help         Prints help information
    -V, --version      Prints version information
```

## How to install

### Using cargo
You need to install cargo on your system through your package manager or any other means.

Then you simply install it through cargo.

```
$ > cargo install linear_clock
```

### Using source
You need to install cargo on your system through your package manager or any other means.

Then  you download the repository through git or manual.

After unpacking or downloading from git you have to switch into the folder of rusty-turing-machine.

Then run `cargo install --path .`.

```
$ > cd linear_clock
$ > cargo install --path
```

## License
rusty-turing-machine is dual licensed under [MIT License](LICENSE-MIT) and [Apache 2 License](LICENSE-APACHE).
