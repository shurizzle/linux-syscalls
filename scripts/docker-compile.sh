#!/bin/sh

set -eux

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
	pwd -P
)"

ARCH="$1"
shift

"$SCRIPTPATH/docker-run.sh" "$ARCH" /compile.sh "$@"
