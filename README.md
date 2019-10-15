# relog

[![Build Status](https://cloud.drone.io/api/badges/mkroli/relog/status.svg)](https://cloud.drone.io/mkroli/relog)

## Installation
Download executable from [Releases](https://github.com/mkroli/relog/releases) or build:
```bash
git clone https://github.com/mkroli/relog.git
cd relog
cargo install
```

## Usage
```bash
$ relog --help
relog 0.1.0
Michael Krolikowski <mkroli@yahoo.de>
replays logfiles

USAGE:
    relog [OPTIONS] <logfile>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --regex <regex>      regex to extract date from log messages (default is generated from date format)
    -f, --format <format>    date format of logfile as specified in strftime(3) [default: %Y-%m-%d %H:%M:%S]

ARGS:
    <logfile>    logfile to read
```
