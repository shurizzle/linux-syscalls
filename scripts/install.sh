#!/bin/sh

set -eux

case "$1" in
x86)
	MARCH="i686"
	;;
x86_64)
	MARCH="x86-64"
	;;
aarch64)
	MARCH="aarch64"
	;;
arm)
	MARCH="arm"
	;;
riscv64)
	MARCH="riscv64"
	;;
powerpc64)
	MARCH="powerpc64"
	;;
mips)
	MARCH="mips"
	;;
mips64)
	MARCH="mips64"
	;;
s390x)
	MARCH="s390x"
	;;
*)
	echo "arch $1 unsupported" >&2
	exit 1
	;;
esac
shift

case "$MARCH" in
arm)
	ABI="gnueabi"
	;;
mips64)
	ABI="gnuabi64"
	;;
*)
	ABI=gnu
	;;
esac

apt-get -y update
apt-get -y upgrade
apt-get -y install binutils-"$MARCH"-linux-"$ABI"
