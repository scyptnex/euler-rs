#! /usr/bin/env bash

set -eu

main()
{
    local latest=$(ls -t src/bin | grep "^e[0-9]*.rs$" | head -n 1)
    latest="${latest%.rs}"
    if [ $# -gt 0 -a $1 == "--time" ]; then
        cargo build --release --bin "${latest}"
        time ./target/release/"${latest}"
    else
        cargo test --bin "${latest}"
        cargo run --bin "${latest}"
    fi
}

main "$@"
