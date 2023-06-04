#!/bin/sh

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
	pwd -P
)"

set -eux

ARCH="$1"
shift

"$SCRIPTPATH/docker-run.sh" "$ARCH" /compile.sh "$@"
