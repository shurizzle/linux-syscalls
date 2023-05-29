#!/bin/sh

set -eux

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1
	pwd -P
)"

docker buildx build \
	-f "$SCRIPTPATH/../docker/compile.dockerfile" \
	-t "linux-syscalls/$1" \
	--build-arg ARCH="$1" \
	"$SCRIPTPATH/.."
