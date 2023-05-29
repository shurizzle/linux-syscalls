#!/bin/sh

set -eux

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
	pwd -P
)"

ARCH="$1"
shift

if ! (docker images --format '{{.Repository}}' | grep "^linux-syscalls/$ARCH\$"); then
	"$SCRIPTPATH/docker-create.sh" "$ARCH"
fi

docker run --rm \
	--user "$(id -u):$(id -g)" \
	-v "$(realpath "${SCRIPTPATH}/../src/outline"):/project" \
	"linux-syscalls/$ARCH" "$@"
