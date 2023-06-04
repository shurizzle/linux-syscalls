#!/bin/sh

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
	pwd -P
)"

set -eux

exec "$SCRIPTPATH/docker-run.sh" "$1" ./scripts/test.sh "$@"
