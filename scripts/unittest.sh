#!/bin/bash
cargo +stable test --target $(rustc -vV | grep host | cut -d ' ' -f2) -p va416xx-hal --features alloc
