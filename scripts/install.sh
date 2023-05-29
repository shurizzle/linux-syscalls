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
	MARCH="$1"
	;;
*)
	echo "arch $1 unsupported" >&2
	exit 1
	;;
esac
shift

apt-get -y update
apt-get -y upgrade
apt-get -y install binutils-"$MARCH"-linux-gnu
