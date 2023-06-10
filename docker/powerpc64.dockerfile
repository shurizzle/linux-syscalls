FROM ubuntu:22.04
ARG ARCH
ENV ARCH="$ARCH"

ENV PATH="${PATH}:/rust/bin"

ADD /scripts/install.sh /install.sh

RUN chmod +x /install.sh && \
  env ENVFILE=/env.sh /install.sh "${ARCH}" && \
  rm -f /install.sh && \
  mkdir -p /target && \
  apt-get clean autoclean && \
  apt-get autoremove --yes && \
  rm -rf /var/cache/apt/archives && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /target
CMD ["/bin/bash"]

ENV CC_powerpc64_unknown_linux_gnu='powerpc64-linux-gnu-gcc'
ENV CXX_powerpc64_unknown_linux_gnu='powerpc64-linux-gnu-g++'
ENV AR_powerpc64_unknown_linux_gnu='powerpc64-linux-gnu-ar'
ENV CARGO_TARGET_POWERPC64_UNKNOWN_LINUX_GNU_LINKER='powerpc64-linux-gnu-gcc'
ENV CARGO_TARGET_POWERPC64_UNKNOWN_LINUX_GNU_RUNNER='qemu-ppc64 -L /usr/powerpc64-linux-gnu'
ENV CC_powerpc64le_unknown_linux_gnu='powerpc64le-linux-gnu-gcc'
ENV CXX_powerpc64le_unknown_linux_gnu='powerpc64le-linux-gnu-g++'
ENV AR_powerpc64le_unknown_linux_gnu='powerpc64le-linux-gnu-ar'
ENV CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_GNU_LINKER='powerpc64le-linux-gnu-gcc'
ENV CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_GNU_RUNNER='qemu-ppc64le -L /usr/powerpc64le-linux-gnu'
ENV CC_powerpc64_unknown_linux_musl='powerpc64-linux-musl-gcc'
ENV CXX_powerpc64_unknown_linux_musl='powerpc64-linux-musl-g++'
ENV AR_powerpc64_unknown_linux_musl='powerpc64-linux-musl-ar'
ENV CARGO_TARGET_POWERPC64_UNKNOWN_LINUX_MUSL_LINKER='powerpc64-linux-musl-gcc'
ENV CARGO_TARGET_POWERPC64_UNKNOWN_LINUX_MUSL_RUNNER='qemu-ppc64 -L /usr/powerpc64-linux-musl'
ENV CC_powerpc64le_unknown_linux_musl='powerpc64le-linux-musl-gcc'
ENV CXX_powerpc64le_unknown_linux_musl='powerpc64le-linux-musl-g++'
ENV AR_powerpc64le_unknown_linux_musl='powerpc64le-linux-musl-ar'
ENV CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_MUSL_LINKER='powerpc64le-linux-musl-gcc'
ENV CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_MUSL_RUNNER='qemu-ppc64le -L /usr/powerpc64le-linux-musl'
