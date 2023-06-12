FROM ubuntu:22.04
ARG ARCH
ENV ARCH="$ARCH"

ENV PATH="${PATH}:/rust/bin"
ENV RUST_COMPILER_RT_ROOT=/opt/compiler-rt

ADD /scripts/install.sh /install.sh
ADD /docker/entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh && \
  chmod +x /install.sh && \
  env ENVFILE=/env.sh /install.sh "${ARCH}" && \
  rm -f /install.sh && \
  mkdir -p /target && \
  apt-get clean autoclean && \
  apt-get autoremove --yes && \
  rm -rf /var/cache/apt/archives && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /target
ENTRYPOINT ["/entrypoint.sh"]
CMD ["/bin/bash"]

ENV CC_mips_unknown_linux_gnu='mips-linux-gnu-gcc'
ENV CXX_mips_unknown_linux_gnu='mips-linux-gnu-g++'
ENV AR_mips_unknown_linux_gnu='mips-linux-gnu-ar'
ENV CARGO_TARGET_MIPS_UNKNOWN_LINUX_GNU_LINKER='mips-linux-gnu-gcc'
ENV CARGO_TARGET_MIPS_UNKNOWN_LINUX_GNU_RUNNER='qemu-mips -L /usr/mips-linux-gnu'
ENV CC_mips_unknown_linux_musl='mips-linux-musl-gcc'
ENV CXX_mips_unknown_linux_musl='mips-linux-musl-g++'
ENV AR_mips_unknown_linux_musl='mips-linux-musl-ar'
ENV CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_LINKER='mips-linux-musl-gcc'
ENV CARGO_TARGET_MIPS_UNKNOWN_LINUX_MUSL_RUNNER='qemu-mips -L /usr/mips-linux-musl'
ENV CC_mipsel_unknown_linux_gnu='mipsel-linux-gnu-gcc'
ENV CXX_mipsel_unknown_linux_gnu='mipsel-linux-gnu-g++'
ENV AR_mipsel_unknown_linux_gnu='mipsel-linux-gnu-ar'
ENV CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_GNU_LINKER='mipsel-linux-gnu-gcc'
ENV CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_GNU_RUNNER='qemu-mipsel -L /usr/mipsel-linux-gnu'
ENV CC_mipsel_unknown_linux_musl='mipsel-linux-musl-gcc'
ENV CXX_mipsel_unknown_linux_musl='mipsel-linux-musl-g++'
ENV AR_mipsel_unknown_linux_musl='mipsel-linux-musl-ar'
ENV CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_MUSL_LINKER='mipsel-linux-musl-gcc'
ENV CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_MUSL_RUNNER='qemu-mipsel -L /usr/mipsel-linux-musl'
