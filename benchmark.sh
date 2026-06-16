#!/bin/bash

if ! command -v hyperfine >/dev/null 2>&1
then
    sudo apt install hyperfine -y
fi

cargo build --release
hyperfine --warmup 10 -n "CSV Generator" -r 100 './run.sh'