#!/bin/bash

set -eux

ARCH="$1"
shift

remove_wget=false
if ! which wget 2>/dev/null >/dev/null; then
	remove_wget=true
fi
apt-get -y update
apt-get -y upgrade
apt-get -y install build-essential wget

set_env() {
	local triple="$1" \
		prefix="$2" \
		runner="$3" \
		lcase_triple \
		ucase_triple

	lcase_triple="$(printf %s "$triple" | tr '-' '_')"
	ucase_triple="$(printf %s "$triple" | tr 'a-z-' 'A-Z_')"

	(
		echo "CC_${lcase_triple}=\"${prefix}gcc\""
		echo "CXX_${lcase_triple}=\"${prefix}g++\""
		echo "AR_${lcase_triple}=\"${prefix}ar\""
		echo "CARGO_TARGET_${ucase_triple}_LINKER=\"${prefix}gcc\""
		echo "CARGO_TARGET_${ucase_triple}_RUNNER=\"$runner\""
	) >>"$ENVFILE"
}

setup_loongarch64() {
	cd

	# install toolchain
	wget https://github.com/loongson/build-tools/releases/download/2022.09.06/loongarch64-clfs-6.3-cross-tools-gcc-glibc.tar.xz
	tar xf loongarch64-clfs-6.3-cross-tools-gcc-glibc.tar.xz
	mkdir -p /opt
	mv cross-tools /opt/loongarch64-unknown-linux-gnu
	rm -f loongarch64-clfs-6.3-cross-tools-gcc-glibc.tar.xz

	# install qemu
	wget https://github.com/loongson/build-tools/releases/download/2022.09.06/qemu-loongarch64
	chmod +x qemu-loongarch64
	mv qemu-loongarch64 /bin/qemu-loongarch64

	local triple="loongarch64-unknown-linux-gnu"
	rustup target add loongarch64-unknown-linux-gnu --toolchain nightly

	set_env "$triple" \
		/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu- \
		"env LD_LIBRARY_PATH=/lib64 qemu-loongarch64 -L /opt/loongarch64-unknown-linux-gnu/target/usr"
	for tool in addr2line ar as c++ c++filt cpp elfedit g++ gcc gcc-ar gcc-nm \
		gcc-ranlib gcov gcov-dump gcov-tool gprof ld ld.bfd lto-dump nm objcopy \
		objdump ranlib readelf size strings strip; do
		ln -s "/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-$tool" "/bin/loongarch64-linux-gnu-$tool"
	done
}

setup_x86_64() {
	local libc

	apt-get install -y qemu-user gcc-x86-64-linux-gnu

	for libc in gnu; do
		rustup target add "x86_64-unknown-linux-${libc}"
	done
}

setup_x86() {
	local arch libc triple

	apt-get install -y qemu-user gcc-i686-linux-gnu

	for arch in i686 i586; do
		for libc in gnu; do
			triple="${arch}-unknown-linux-${libc}"
			rustup target add "$triple"

			set_env "$triple" \
				"${arch}-linux-${libc}-" \
				"qemu-i386 -L /usr/i686-linux-${libc}"
		done
	done
}

setup_arm() {
	local arch libc triple

	apt-get install -y qemu-user gcc-arm-linux-gnueabi{,hf}

	for arch in arm armv5te armv7; do
		for libc in gnu; do
			triple="${arch}-unknown-linux-${libc}eabi"
			rustup target add "$triple"

			set_env "$triple" \
				"arm-linux-${libc}eabi-" \
				"qemu-arm -L /usr/arm-linux-${libc}eabi"
		done
	done

	for arch in arm armv7 thumbv7neon; do
		for libc in gnu; do
			triple="${arch}-unknown-linux-${libc}eabihf"
			rustup target add "$triple"

			set_env "$triple" \
				"arm-linux-${libc}eabihf-" \
				"qemu-arm -L /usr/arm-linux-${libc}eabihf"
		done
	done
}

setup_aarch64() {
	local libc triple

	apt-get -y install -y qemu-user gcc-aarch64-linux-gnu

	for libc in gnu; do
		triple="aarch64-unknown-linux-${libc}"

		rustup target add "$triple"

		set_env "$triple" \
			"aarch64-linux-${libc}-" \
			"qemu-aarch64 -L /usr/aarch64-linux-${libc}"
	done
}

setup_riscv64() {
	local libc triple

	apt-get install -y qemu-user gcc-riscv64-linux-gnu

	for libc in gnu; do
		triple="riscv64gc-unknown-linux-${libc}"
		rustup target add "$triple"

		set_env "$triple" \
			"riscv64-linux-${libc}-" \
			"qemu-riscv64 -L /usr/riscv64-linux-${libc}"
	done
}

setup_powerpc64() {
	local libc triple

	apt-get install -y qemu-user gcc-powerpc64-linux-gnu

	for libc in gnu; do
		triple="powerpc64-unknown-linux-${libc}"
		rustup target add "$triple"

		set_env "$triple" \
			"powerpc64-linux-${libc}-" \
			"qemu-ppc64 -L /usr/powerpc64-linux-${libc}"
	done
}

setup_mips() {
	local libc triple

	apt-get install -y qemu-user gcc-mips-linux-gnu

	for libc in gnu; do
		triple="mips-unknown-linux-${libc}"
		rustup target add "$triple"

		set_env "$triple" \
			"mips-linux-${libc}-" \
			"qemu-mips -L /usr/mips-linux-${libc}"
	done
}

setup_mips64() {
	local libc triple

	apt-get install -y qemu-user gcc-mips64-linux-gnuabi64

	for libc in gnu; do
		triple="mips64-unknown-linux-${libc}abi64"
		rustup target add "$triple"

		set_env "$triple" \
			"mips64-linux-${libc}abi64-" \
			"qemu-mips64 -L /usr/mips-linux-${libc}abi64"
	done
}

setup_s390x() {
	local libc triple

	apt-get install -y qemu-user gcc-s390x-linux-gnu

	for libc in gnu; do
		triple="s390x-unknown-linux-${libc}"
		rustup target add "$triple"

		set_env "$triple" \
			"s390x-linux-${libc}-" \
			"qemu-s390x -L /usr/s390x-linux-${libc}"
	done
}

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
		echo 'PATH="$PATH:/opt/rust/bin"'
	) >>"$ENVFILE"

	rm -f rustup-init
fi
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

"setup_${ARCH}"

if $remove_wget; then
	apt-get -y purge wget
fi
apt-get -y autoremove
