#! /usr/bin/env bash

set -eux

main()
{
    local latest=$(ls -t src/bin | grep "^e[0-9]*.rs$" | head -n 1)
    latest="${latest%.rs}"
    cargo test --bin "${latest}"
    cargo run --bin "${latest}"
}

main "$@"
