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

	(
		cat "${SCRIPTPATH}/../docker/base.dockerfile"
		echo
		"${SCRIPTPATH}/env.sh" "$ARCH" |
			grep -vP '^\s*($|#)' |
			sed -e '/^#/d;/^\s*$/d' \
				-e "s/'/'\\\''/g" \
				-e "s/=\(.*\)/='\1'/g" \
				-e 's/^/ENV /'
	) >"${SCRIPTPATH}/../docker/${ARCH}.dockerfile"

	docker buildx build \
		-f "$SCRIPTPATH/../docker/${ARCH}.dockerfile" \
		-t "shurizzle/toolchain-gnu-musl-cargoenv-${ARCH}:latest" \
		--build-arg ARCH="$ARCH" \
		"$SCRIPTPATH/.."

	docker push "shurizzle/toolchain-gnu-musl-cargoenv-${ARCH}:latest"
done
