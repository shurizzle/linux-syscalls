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

tty=
if [ -t 0 ]; then
	tty=-it
fi

exec docker run --rm $tty \
	--user "$(id -u):$(id -g)" \
	-v "$(realpath "${SCRIPTPATH}/.."):/project" \
	-v "$HOME:$HOME" \
	-e "HOME=$HOME" \
	"linux-syscalls/$ARCH" "$@"
