#!/bin/bash

TOOLCHAIN="$1"
shift

ARCH="$1"
shift

set -eux

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1
	pwd -P
)"
CRATEPATH="$(realpath "${SCRIPTPATH}/..")"

mkdir -p "${CRATEPATH}/target"

tty=
if [ -t 0 ] && [ -t 1 ]; then
	tty=-it
fi

docker run --rm $tty \
	--userns host \
	-e 'CARGO_HOME=/cargo' \
	-e 'CARGO_TARGET_DIR=/target' \
	-e TERM \
	-e "USER=$(id -un)" \
	--user "$(id -u):$(id -g)" \
	-v "${CARGO_HOME:-${HOME}/.cargo}":/cargo:z \
	-v /cargo/bin \
	-v "${CRATEPATH}:/project:z" \
	-v "$(rustc "+${TOOLCHAIN}" --print sysroot)":/rust:z,ro \
	-v "${CRATEPATH}/target:/target:z" \
	-w /project \
	"shurizzle/toolchain-gnu-musl-cargoenv-${ARCH}:latest" "$@"
