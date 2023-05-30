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
*)
	ABI=gnu
	;;
esac

apt-get -y update
apt-get -y upgrade
apt-get -y install binutils-"$MARCH"-linux-"$ABI"
