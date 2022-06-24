# File:        Justfile
# Author:      Zakhary Kaplan <zakharykaplan@gmail.com>
# Created:     27 Apr 2022
# SPDX-License-Identifier: MIT OR Apache-2.0

alias b := build
alias r := run
alias t := test

# default recipe
_: help

# build all artifacts
all: build doc release

# compile local package
build: dev

# clean build artifacts
clean:
    @cargo clean

# build `dev` profile
dev:
    @cargo build --all-targets

# apply lints
fix: && fmt
    @cargo clippy --workspace --fix --allow-dirty --allow-staged

# format source files
fmt:
    @cargo +nightly fmt

# document source files
doc:
    @cargo doc

# list available recipes
help:
    @just --list

# lint source files
lint:
    @cargo clippy --workspace --all-targets

# build `release` profile
release:
    @cargo build --all-targets --release

# run binary
run rom:
    @cargo run --release -- --check "{{ rom }}"

# perform tests
test *opts:
    @cargo test --workspace -- {{ opts }}

# vim:fdl=0:fdm=marker:ft=make:
