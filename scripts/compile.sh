#!/bin/sh

set -eux

if [ -e "/env.sh" ]; then
	. /env.sh
fi

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
mips64)
	ABI=gnuabi64
	;;
esac

T="$(mktemp -d)"

"$MARCH"-linux-"$ABI"-as -g "$@" -o "$T/$ARCH-d.o" "src/outline/$FARCH.s"
mkdir -p "stc/outline/debug/$ARCH"
"$MARCH"-linux-"$ABI"-ar -crs "src/outline/debug/$ARCH/liblinux_syscalls_rs.a" "$T/$ARCH-d.o"

"$MARCH"-linux-"$ABI"-as "$@" -o "$T/$ARCH-r.o" "src/outline/$FARCH.s"
mkdir -p "src/outline/release/$ARCH"
"$MARCH"-linux-"$ABI"-ar -crs "src/outline/release/$ARCH/liblinux_syscalls_rs.a" "$T/$ARCH-r.o"
