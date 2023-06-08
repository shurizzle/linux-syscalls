#!/bin/bash

set -eux

ARCH="$1"
shift

sudo=
if [ "${CI:-false}" = true ]; then
	sudo=sudo
fi

$sudo apt-get update -y
if [ "${CI:-false}" != true ]; then
	$sudo apt-get upgrade -y
fi

install_wget() {
	if ! which wget 2>/dev/null >/dev/null; then
		$sudo apt-get install -y wget
		$sudo apt-mark auto wget
	fi
}

common_install() {
	local mark_wget=false
	if ! which wget 2>/dev/null >/dev/null; then
		mark_wget=true
	fi

	local mark_git=false
	if ! which git 2>/dev/null >/dev/null; then
		mark_git=true
	fi

	local mark_file=false
	if ! which file 2>/dev/null >/dev/null; then
		mark_file=true
	fi

	local mark_texinfo=false
	if ! which makeinfo 2>/dev/null >/dev/null; then
		mark_texinfo=true
	fi

	$sudo apt-get install -y build-essential wget git file texinfo "$@"
	if $mark_wget; then
		$sudo apt-mark auto wget
	fi
	if $mark_git; then
		$sudo apt-mark auto git
	fi
	if $mark_file; then
		$sudo apt-mark auto file
	fi
	if $mark_texinfo; then
		$sudo apt-mark auto texinfo
	fi
}

install_musl() {
	MUSLDIR="$(mktemp -d)"

	git clone --depth=1 https://github.com/richfelker/musl-cross-make.git "$MUSLDIR"
	cd "$MUSLDIR"
	cat <<EOF >config.mak
TARGET = $1
OUTPUT = /usr
GCC_VER = 11.2.0
BINUTILS_VER = 2.33.1
GCC_CONFIG += --enable-default-pie
EOF
	make -j"$(nproc)" install

	cd
	rm -rf "$MUSLDIR"

	local sysroot="/usr/$1"
	local arch="$2"
	local ld_arch="${arch//_/-}"
	local dst

	mkdir -p "${sysroot}/usr/lib"
	local src="${sysroot}/lib/libc.so"

	for dst in \
		"/lib/ld-musl-${arch}.so" \
		"/lib/ld-musl-${arch}.so.1" \
		"${sysroot}/lib/ld-musl-${arch}.so" \
		"${sysroot}/lib/ld-musl-${arch}.so.1" \
		"${sysroot}/usr/lib/ld.so" \
		"${sysroot}/usr/lib/ld.so.1" \
		"${sysroot}/usr/lib/libc.so" \
		"${sysroot}/usr/lib/libc.so.1" \
		"${sysroot}/lib/ld-linux-${ld_arch}.so" \
		"${sysroot}/lib/ld-linux-${ld_arch}.so.1"; do
		if [ -L "$dst" ] && [ ! -a "$dst" ]; then
			ln -sf "${src}" "${dst}"
		elif [ ! -f "$dst" ]; then
			ln -s "${src}" "${dst}"
		fi
	done

	rm -f "${sysroot}/lib/libstdc++.so"*

	echo 'GROUP ( libstdc++.a AS_NEEDED( -lgcc -lc -lm ) )' >"${sysroot}/lib/libstdc++.so.6.0.29"
	ln -s libstdc++.so.6.0.29 "${sysroot}/lib/libstdc++.so.6"
	ln -s libstdc++.so.6.0.29 "${sysroot}/lib/libstdc++.so"

	echo "${sysroot}/lib" >>"/etc/ld-musl-${arch}.path"
}

musl_linker() {
	cat <<EOF
#!/bin/bash

set -x
set -euo pipefail

if [ \$# -eq 0 ] || [ "\$(rustc -Vv | grep '^release:' | cut -d: -f2 | cut -d. -f2)" -ge "$2" ]; then
  exec "${1}gcc" "\$@"
else
  exec "${1}gcc" "\$@" -lgcc -static-libgcc
fi
EOF
}

set_env() {
	local triple="$1" \
		prefix="$2" \
		runner="$3" \
		lcase_triple \
		ucase_triple

	lcase_triple="$(printf %s "$triple" | tr '-' '_')"
	ucase_triple="$(printf %s "$triple" | tr 'a-z-' 'A-Z_')"

	(
		echo "CC_${lcase_triple}=${prefix}gcc"
		echo "CXX_${lcase_triple}=${prefix}g++"
		echo "AR_${lcase_triple}=${prefix}ar"
		echo "CARGO_TARGET_${ucase_triple}_LINKER=${prefix}gcc"
		echo "CARGO_TARGET_${ucase_triple}_RUNNER=$runner"
	) >>"$ENVFILE"
}

setup_loongarch64() {
	cd

	common_install

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

	# rustup target add loongarch64-unknown-linux-gnu --toolchain nightly

	for tool in addr2line ar as c++ c++filt cpp elfedit g++ gcc gcc-ar gcc-nm \
		gcc-ranlib gcov gcov-dump gcov-tool gprof ld ld.bfd lto-dump nm objcopy \
		objdump ranlib readelf size strings strip; do
		ln -s "/opt/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-$tool" \
			"/bin/loongarch64-linux-gnu-$tool"
	done
}

setup_x86_64() {
	common_install qemu-user musl-tools gcc-x86-64-linux-gnu
	# rustup toolchain install 1.59.0

	# for libc in gnu musl; do
	# 	rustup target add "x86_64-unknown-linux-${libc}" --toolchain 1.40.0
	# 	rustup target add "x86_64-unknown-linux-${libc}" --toolchain 1.59.0
	# 	rustup target add "x86_64-unknown-linux-${libc}" --toolchain nightly
	# done
}

setup_x86() {
	common_install qemu-user gcc-i686-linux-gnu
	install_musl i686-linux-musl i686
	# rustup toolchain install 1.59.0

	# for arch in i686 i586; do
	# 	for libc in gnu musl; do
	# 		triple="${arch}-unknown-linux-${libc}"
	# 		rustup target add "$triple" --toolchain 1.40.0
	# 		rustup target add "$triple" --toolchain 1.59.0
	# 		rustup target add "$triple" --toolchain nightly
	# 	done
	# done
}

setup_arm() {
	common_install qemu-user gcc-arm-linux-gnueabi{,hf}
	install_musl arm-linux-musleabi arm
	install_musl arm-linux-musleabihf armhf
	musl_linker arm-linux-musleabi- 65 >/armv5te-musl.sh
	chmod +x /armv5te-musl.sh
	# rustup toolchain install 1.59.0

	# for arch in arm armv5te armv7; do
	# 	for libc in gnu musl; do
	# 		triple="${arch}-unknown-linux-${libc}eabi"
	# 		rustup target add "$triple" --toolchain 1.40.0
	# 		rustup target add "$triple" --toolchain 1.59.0
	# 		rustup target add "$triple" --toolchain nightly
	#
	# 		set_env "$triple" \
	# 			"arm-linux-${libc}eabi-" \
	# 			"qemu-arm -L /usr/arm-linux-${libc}eabi"
	# 	done
	# done

	# for arch in arm armv7 thumbv7neon; do
	# 	for libc in gnu musl; do
	# 		triple="${arch}-unknown-linux-${libc}eabihf"
	# 		rustup target add "$triple" --toolchain 1.40.0
	# 		rustup target add "$triple" --toolchain 1.59.0
	# 		rustup target add "$triple" --toolchain nightly
	#
	# 		set_env "$triple" \
	# 			"arm-linux-${libc}eabihf-" \
	# 			"qemu-arm -L /usr/arm-linux-${libc}eabihf"
	# 	done
	# done
}

setup_aarch64() {
	common_install qemu-user gcc-aarch64-linux-gnu
	install_musl aarch64-linux-musl aarch64
	musl_linker aarch64-linux-musl- 48 >/aarch64-musl.sh
	chmod +x /aarch64-musl.sh
	# rustup toolchain install 1.59.0

	# for libc in gnu musl; do
	# 	triple="aarch64-unknown-linux-${libc}"
	#
	# 	rustup target add "$triple" --toolchain 1.40.0
	# 	rustup target add "$triple" --toolchain 1.59.0
	# 	rustup target add "$triple" --toolchain nightly
	#
	# 	set_env "$triple" \
	# 		"aarch64-linux-${libc}-" \
	# 		"qemu-aarch64 -L /usr/aarch64-linux-${libc}"
	# done
}

setup_riscv64() {
	common_install qemu-user gcc-riscv64-linux-gnu
	install_musl riscv64-linux-musl riscv64
	# rustup toolchain install 1.59.0

	# for libc in gnu musl; do
	# 	triple="riscv64gc-unknown-linux-${libc}"
	# 	rustup target add "$triple" --toolchain 1.42.0
	# 	rustup target add "$triple" --toolchain 1.59.0
	# 	rustup target add "$triple" --toolchain nightly
	#
	# 	set_env "$triple" \
	# 		"riscv64-linux-${libc}-" \
	# 		"qemu-riscv64 -L /usr/riscv64-linux-${libc}"
	# done
}

setup_powerpc() {
	common_install qemu-user gcc-powerpc-linux-gnu
	install_musl powerpc-linux-musl powerpc
	# rustup toolchain install 1.59.0

	# for libc in gnu musl; do
	# 	triple="powerpc-unknown-linux-${libc}"
	# 	rustup target add "$triple" --toolchain 1.40.0
	# 	rustup target add "$triple" --toolchain 1.59.0
	# 	rustup target add "$triple" --toolchain nightly
	#
	# 	set_env "$triple" \
	# 		"powerpc-linux-${libc}-" \
	# 		"qemu-ppc -L /usr/powerpc-linux-${libc}"
	# done
}

setup_powerpc64() {
	common_install qemu-user gcc-powerpc64-linux-gnu \
		gcc-powerpc64le-linux-gnu
	install_musl powerpc64-linux-musl powerpc64
	install_musl powerpc64le-linux-musl powerpc64le
	# rustup toolchain install 1.59.0

	# for libc in gnu musl; do
	# 	triple="powerpc64-unknown-linux-${libc}"
	# 	rustup target add "$triple" --toolchain 1.40.0
	# 	rustup target add "$triple" --toolchain 1.59.0
	# 	rustup target add "$triple" --toolchain nightly
	#
	# 	set_env "$triple" \
	# 		"powerpc64-linux-${libc}-" \
	# 		"qemu-ppc64 -L /usr/powerpc64-linux-${libc}"
	#
	# 	triple="powerpc64le-unknown-linux-${libc}"
	# 	rustup target add "$triple" --toolchain 1.40.0
	# 	rustup target add "$triple" --toolchain 1.59.0
	# 	rustup target add "$triple" --toolchain nightly
	#
	# 	set_env "$triple" \
	# 		"powerpc64le-linux-${libc}-" \
	# 		"qemu-ppc64le -L /usr/powerpc64le-linux-${libc}"
	# done
}

setup_mips() {
	common_install qemu-user gcc-mips-linux-gnu gcc-mipsel-linux-gnu
	install_musl mips-linux-musl mips
	install_musl mipsel-linux-musl mipsel
	# rustup toolchain install 1.59.0

	# for arch in mips mipsel; do
	# 	for libc in gnu musl; do
	# 		triple="${arch}-unknown-linux-${libc}"
	# 		rustup target add "$triple" --toolchain 1.40.0
	# 		rustup target add "$triple" --toolchain 1.59.0
	# 		rustup target add "$triple" --toolchain nightly
	#
	# 		set_env "$triple" \
	# 			"${arch}-linux-${libc}-" \
	# 			"qemu-${arch} -L /usr/${arch}-linux-${libc}"
	# 	done
	# done
}

setup_mips64() {
	common_install qemu-user gcc-mips64-linux-gnuabi64 \
		gcc-mips64el-linux-gnuabi64
	install_musl mips64-linux-muslabi64 mips64
	install_musl mips64el-linux-muslabi64 mips64el
	musl_linker mips64-linux-muslabi64- 65 >/mips64-musl.sh
	chmod +x /mips64-musl.sh
	musl_linker mips64el-linux-muslabi64- 65 >/mips64el-musl.sh
	chmod +x /mips64el-musl.sh
	# rustup toolchain install 1.59.0

	# for arch in mips64 mips64el; do
	# 	for libc in gnu musl; do
	# 		triple="${arch}-unknown-linux-${libc}abi64"
	# 		rustup target add "$triple" --toolchain 1.40.0
	# 		rustup target add "$triple" --toolchain 1.59.0
	# 		rustup target add "$triple" --toolchain nightly
	#
	# 		set_env "$triple" \
	# 			"${arch}-linux-${libc}abi64-" \
	# 			"qemu-${arch} -L /usr/${arch}-linux-${libc}abi64"
	# 	done
	# done
}

setup_s390x() {
	common_install qemu-user gcc-s390x-linux-gnu
	install_musl s390x-linux-musl s390x
	# rustup toolchain install 1.59.0

	# for libc in gnu musl; do
	# 	triple="s390x-unknown-linux-${libc}"
	# 	rustup target add "$triple" --toolchain 1.40.0
	# 	rustup target add "$triple" --toolchain 1.59.0
	# 	rustup target add "$triple" --toolchain nightly
	#
	# 	set_env "$triple" \
	# 		"s390x-linux-${libc}-" \
	# 		"qemu-s390x -L /usr/s390x-linux-${libc}"
	# done
}

# TODO: riscv32

"setup_${ARCH}"
