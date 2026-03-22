#! /usr/bin/env bash

set -eu

filefor ()
{
    printf "src/bin/e%03d.rs\n" $1
}

main()
{
    local NXT=1
    while [[ -e $(filefor $NXT) ]]; do
        NXT=$((NXT+1))
    done
    cp src/bin/template.rs $(filefor $NXT)
    xdg-open "https://projecteuler.net/problem=${NXT}"
    v $(filefor $NXT)
}

main "$@"
