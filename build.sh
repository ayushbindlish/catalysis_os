#!/usr/bin/env bash
# Script to build free standing binary
cargo rustc -- -C link-arg=-nostartfiles