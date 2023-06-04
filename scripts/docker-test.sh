#!/bin/sh

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
	pwd -P
)"

set -eux

if [ $# -eq 0 ]; then
	set -- arm loongarch64 x86_64 x86 aarch64 riscv64 powerpc64 mips mips64 s390x
fi

while [ $# -ne 0 ]; do
	"$SCRIPTPATH/docker-run.sh" "$1" ./scripts/test.sh "$1"
	shift
done
