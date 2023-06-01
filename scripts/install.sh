#!/bin/sh

set -eux

ARCH="$1"

case "$ARCH" in
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
loongarch64)
	MARCH="loongarch64"
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
apt-get -y install build-essential wget

if [ $MARCH = loongarch64 ]; then
	cd
	wget https://github.com/loongson/build-tools/releases/download/2022.09.06/loongarch64-clfs-6.3-cross-tools-gcc-glibc.tar.xz
	tar xf loongarch64-clfs-6.3-cross-tools-gcc-glibc.tar.xz
	mkdir -p /opt
	mv cross-tools /opt/loongarch64-unknown-linux-gnu
	rm -f loongarch64-clfs-6.3-cross-tools-gcc-glibc.tar.xz
	wget https://github.com/loongson/build-tools/releases/download/2022.09.06/qemu-loongarch64
	chmod +x qemu-loongarch64
	mv qemu-loongarch64 /bin/qemu-loongarch64
	(
		echo CC_loongarch64_unknown_linux_gnu=/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-gcc
		echo CXX_loongarch64_unknown_linux_gnu=/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-g++
		echo AR_loongarch64_unknown_linux_gnu=/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-gcc-ar
		echo CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_LINKER=/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-gcc
		echo export CC_loongarch64_unknown_linux_gnu
		echo export CXX_loongarch64_unknown_linux_gnu
		echo export AR_loongarch64_unknown_linux_gnu
		echo export CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_LINKER
		echo QEMU_LD_PREFIX=/opt/loongarch64-unknown-linux-gnu/target/usr
		echo 'PATH="/opt/loongarch64-unknown-linux-gnu/bin:$PATH"'
		echo 'CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_RUNNER="env LD_LIBRARY_PATH="/lib64" qemu-loongarch64"'
		echo export QEMU_LD_PREFIX
		echo export CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_RUNNER
	) >>/env.sh
	for tool in addr2line ar as c++ c++filt cpp elfedit g++ gcc gcc-ar gcc-nm \
		gcc-ranlib gcov gcov-dump gcov-tool gfortran gprof ld ld.bfd lto-dump nm \
		objcopy objdump ranlib readelf size strings strip; do
		ln -s "/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-$tool" "/bin/loongarch64-linux-gnu-$tool"
	done
else
	if [ "$MARCH" = i686 ]; then
		apt-get -y install libc-dev-"$MARCH"-cross
	else
		apt-get -y install gcc-"$MARCH"-linux-"$ABI" qemu-user
	fi
fi

if ! which rustup 2>/dev/null >/dev/null; then
	RUSTUP_HOME="/opt/rust"
	export RUSTUP_HOME
	CARGO_HOME="/opt/rust"
	export CARGO_HOME
	cd
	wget https://sh.rustup.rs -O rustup-init
	chmod +x rustup-init
	./rustup-init -y --no-modify-path
	PATH="$PATH:/opt/rust/bin"
	(
		echo 'RUSTUP_HOME="/opt/rust"'
		echo 'export RUSTUP_HOME'
		echo 'PATH="$PATH:/opt/rust/bin"'
	) >>/env.sh

	rm -f rustup-init
fi

apt-get -y purge wget
apt-get -y autoremove

rustup toolchain install nightly
if [ "$ARCH" = loongarch64 ]; then
	rustup target add loongarch64-unknown-linux-gnu --toolchain nightly
elif [ "$ARCH" = riscv64 ]; then
	rustup target add riscv64gc-unknown-linux-"$ABI"
elif [ "$ARCH" = x86 ]; then
	rustup target add "$MARCH"-unknown-linux-"$ABI"
else
	rustup target add "$ARCH"-unknown-linux-"$ABI"
fi
