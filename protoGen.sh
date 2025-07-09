#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

rm -rf volo-gen src/lib.rs
volo init --includes=./proto grpc ./proto/api.proto
bun i
#~/i18n/lib/volo_regen/src/volo_regen.js
bun x volo_regen
cargo fmt
