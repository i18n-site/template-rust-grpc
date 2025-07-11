#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*}
cd $DIR
set -ex
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=git

cargo update
cargo upgrade -i --recursive --verbose
ncu -u && bun i
