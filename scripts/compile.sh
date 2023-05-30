#!/bin/sh

set -eux

MARCH="$ARCH"
FARCH="$ARCH"
ABI=gnu

case "$MARCH" in
x86)
	MARCH="i686"
	;;
esac

case "$MARCH" in
arm)
	ABI=gnueabi
	;;
esac

T="$(mktemp -d)"

"$MARCH"-linux-"$ABI"-as -g "$@" -o "$T/$ARCH-d.o" "$FARCH.s"
mkdir -p "debug/$ARCH"
"$MARCH"-linux-"$ABI"-ar -crs "debug/$ARCH/liblinux_syscalls_rs.a" "$T/$ARCH-d.o"

"$MARCH"-linux-"$ABI"-as "$@" -o "$T/$ARCH-r.o" "$FARCH.s"
mkdir -p "release/$ARCH"
"$MARCH"-linux-"$ABI"-ar -crs "release/$ARCH/liblinux_syscalls_rs.a" "$T/$ARCH-r.o"
