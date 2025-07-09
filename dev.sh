#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
. sh/pid.sh

set -a
. sh/env.sh
set +a

set -ex

# onexit() {
#   docker compose -p $name down
# }
#
# trap onexit EXIT

exec watchexec \
  -r \
  --project-origin . -w src/ \
  --exts rs,toml \
  -- "cargo run -p grpc -- $@"

# ./docker/up.sh -d

# exec mise exec -- bun x concurrently -r --names "kv,srv" \
#   "docker-compose -p $name logs -f" \
#   "./sh/wait-for-it.sh $R_NODE && ./srv/dev.sh"
