#!/bin/sh

set -eux

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1
	pwd -P
)"

if [ $# -eq 0 ]; then
	set -- arm loongarch64 x86_64 x86 aarch64 riscv64 powerpc powerpc64 mips \
		mips64 s390x
fi

while [ $# -ne 0 ]; do
	ARCH="$1"
	shift

	docker buildx build \
		-f "$SCRIPTPATH/../docker/compile.dockerfile" \
		-t "linux-syscalls/$ARCH" \
		--build-arg ARCH="$ARCH" \
		"$SCRIPTPATH/.."
done
