# odot

[![Build Status](https://travis-ci.org/Walther/odot.svg?branch=master)](https://travis-ci.org/Walther/odot)
[![codecov](https://codecov.io/gh/Walther/odot/branch/master/graph/badge.svg)](https://codecov.io/gh/Walther/odot)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

Reverse todo: tool for jotting down the things that have been done.

## CLI Usage

Simple core usecases:

- `odot -m "Created an odot note"`
- `echo "Piped an odot note" | odot`
- `odot --message "An odot note with tags" --tags "comma, separated, tags"`

## Future ideas

- High-performance "something got done" -logger
  - With local saving
  - With synchronizing to somewhere else
- Easy to use as a Rust library; to integrate odot logging to your app
- Easy to use as an NPM library; leverage WASM
