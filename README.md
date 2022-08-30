<!-- omit in TOC -->
# flightaware-rs

> **Rust bindings for Flightaware Firehose API**

## About

This is the library implementing the [Flightaware] Firehose API in [Rust].  It just exports the streaming data into JSON, using the FA API.  It is a more recent implementation than my previous [Flightaware Go].

The `fa-export`) and `fa-tail` utilities are bundled directly as example code.

## Build status

[![Build Status](https://api.cirrus-ci.com/github/keltia/flightaware-rs.svg?branch=main)](https://cirrus-ci.org/keltia/flightaware-rs)
[![Crates.io](https://img.shields.io/crates/v/flightaware-rs.svg)](https://crates.io/crates/docs_rs)
[![Docs](https://img.shields.io/docsrs/flightaware-rs)](https://docs.rs/flightaware-rs)
[![GitHub release](https://img.shields.io/github/release/keltia/flightaware-rs.svg)](https://github.com/keltia/flightaware-rs/releases/)
[![GitHub issues](https://img.shields.io/github/issues/keltia/flightaware-rs.svg)](https://github.com/keltia/flightaware-rs/issues)
[![Rust Version]][Rust 1.56]
[![SemVer](https://img.shields.io/badge/semver-2.0.0-blue)][Semantic Versioning]
[![License](https://img.shields.io/pypi/l/Django.svg)](https://opensource.org/licenses/BSD-2-Clause)

Licensed under the [BSD 2-clause](LICENSE.md) license.

1. [About](#about)
2. [Requirements](#requirements)
2. [Installation](#installation)
3. [Usage](#usage)
4. [API Usage](#api-usage)
5. [References](#references)
6. [TODO](#todo)
7. [MSRV](#msrv)
8. [Contributing](#contributing)
9. [Feedback](#feedback)

## Requirements

This library requires the following cargo modules.

In addition, the utilities consume a few modules such as:

- clap
- serde
- serde_json

### Run-time

### Development

## Installation

Like many Rust utilities & libraries, it is very easy to install:

    cargo install flightaware-rs

that way, you get both the library and its two bundled utilities.

You can also clone the repository and use `cargo install`

    git clone https://github.com/keltia/flightaware-rs
    cargo install

## USAGE

There are two example programs included in `bin/fa-export` and `bin/fa-tail`.  The former is the main driver and the latter is a `tail(1)`-like utility.

### fa-export

```
fa-export -[AOpv] [options...]

Usage of fa-export:
  -A	Autorotate output file
  -B string
    	Begin time for -f pitr|range
  -D string
    	Default destination (NOT IMPL)
  -E string
    	End time for -f range
  -F string
    	Airline filter
  -I string
    	Aircraft Ident filter
  -L string
    	Lat/Long filter
  -O	Overwrite existing file?
  -P string
    	Airport filter
  -X string
    	Hexid output filter
  -d string
    	Stop after N s/mn/h/days
  -e string
    	Events to stream
  -f string
    	Specify which feed we want (default "live")
  -o string
    	Specify output FILE.
  -p	Enable profiling
  -u string
    	Username to connect with
  -v	Set verbose flag.
```

### fa-tail

```
fa-tail -[cv] file

Usage of fa-tail:
  -c	Count records.
  -v	Be verbose
```

The `file` parameter being the file specified by the `-o`option of `fa-export`.

XXX `fa-tail` does not implement most of `tail(1)`options, especially not `-f`.

## API Usage

You start by creating a client instance with your credentials passed as Config
struct. See `fa-export` for a configuration file loading and suff.

 	client := flightaware.NewClient(Config)

Then you can configure the feed type with

 	client.SetFeed(string, []time.Time)

You can also set a timeout time with a value in seconds

 	client.SetTimeout(int64)

You can add one or more different input filters:

    client.AddInputFilter(<type>, <value>)

where type can be one of

     FILTER_EVENT
     FILTER_AIRLINE
     FILTER_IDENT
     FILTER_AIRPORT
     FILTER_LATLONG

The filters you specify will be checked remotely by FlightAware according to the
documentation available at
https://fr.flightaware.com/commercial/firehose/firehose_documentation.rvt

You can specify output filters with using `client.AddOutputFilter(string)`

The default handler is to display all packets.  You can change the default handler
with

 	client.AddHandler(func([]byte)

Last action is to start the consuming/producer loop with

 	client.Start()

Reading will be closed either though getting an EOF from FA or being will killed either manually or through the timeout value.

You can then use

 	client.Close()

to properly close the reading channel.

## References

[Flightaware Firehose]

## License

The [BSD 2-Clause license](https://github.com/keltia/flightaware-rs/LICENSE.md).

## TODO


## MSRV

The Minimum Supported Rust Version is 1.56 due to the 2021 Edition.

The API exposed follows the [Semantic Versioning] 2.0.0 scheme to guarantee a consistent API compatibility.

# Contributing

This project is an open Open Source project, please read `CONTRIBUTING.md`.  It also uses the [Git flow] model where all changes are made in the `develop` branch later incorporated by me in the `main` branch for releases.

1. Fork it ( https://github.com/keltia/dmarc-rs/fork )
2. Checkout the develop branch (`git checkout develop`)
3. Create your feature branch (`git checkout -b my-new-feature`)
4. Commit your changes (`git commit -am 'Add some feature'`)
5. Push to the branch (`git push origin my-new-feature`)
6. Create a new Pull Request

# Feedback

We welcome pull requests, bug fixes and issue reports.

Before proposing a large change, first please discuss your change by raising an issue.

[Flightaware]: http://www.flightaware.com/
[Flightaware Firehose]: https://fr.flightaware.com/commercial/firehose/documentation/summary
[Flightaware Go]: https://github.com/keltia/flightaware-go/
[Git flow]: https://git-flow.readthedocs.io/en/latest/presentation.html
[Rust Version]: https://img.shields.io/badge/Rust%20version-1.56%2B-lightgrey
[Rust 1.56]: https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html
[Rust]: https://rust-lang.org/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html