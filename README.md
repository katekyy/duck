<div>
  <p>
    <a href="#timer-written-in-rust"><img src="https://img.shields.io/github/license/katekyy/duck?color=%2388FF&label=License&style=flat-square"/></a>
    <a href="#timer-written-in-rust"><img src="https://img.shields.io/github/commit-activity/w/katekyy/duck?color=%2388FF&style=flat-square"/></a>
    <img src="/img.png"/>
  </p>
</div>

# Timer written in Rust
It can be used to countdown a start of a livestream, new year or anything else.

## Prerequisites

- [rust]
- [cargo]
- [make]
- [bash]

## Installation
You need to type only one command.
```console
$ git clone https://github.com/katekyy/duck
$ cd duck
$ make
```

## Usage
You can set an message or set it to empty string. Time is formated in: `hh:mm:ss`

```console
$ duck "Some Message" 00:00:15
```

[rust]: https://www.rust-lang.org
[cargo]: https://www.rust-lang.org
[make]: https://www.gnu.org/software/make/
[bash]: https://www.gnu.org/software/bash/