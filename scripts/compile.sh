#!/bin/sh

set -eux

MARCH="$ARCH"
FARCH="$ARCH"

case "$MARCH" in
x86)
	MARCH="i686"
	;;
esac

T="$(mktemp -d)"

"$MARCH"-linux-gnu-as -g "$@" -o "$T/$ARCH-d.o" "$FARCH.s"
mkdir -p "debug/$ARCH"
"$MARCH"-linux-gnu-ar -crs "debug/$ARCH/liblinux_syscalls_rs.a" "$T/$ARCH-d.o"

"$MARCH"-linux-gnu-as "$@" -o "$T/$ARCH-r.o" "$FARCH.s"
mkdir -p "release/$ARCH"
"$MARCH"-linux-gnu-ar -crs "release/$ARCH/liblinux_syscalls_rs.a" "$T/$ARCH-r.o"
