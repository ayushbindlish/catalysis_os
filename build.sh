#!/usr/bin/env bash
# Script to build free standing binary
cargo rustc -- -C link-arg=-nostartfiles # compile for host system
# cargo build --target thumbv7em-none-eabihf # compile for bare metal target

