FROM ubuntu:22.04
ARG ARCH
ENV ARCH="$ARCH"

ADD /scripts/install.sh /install.sh

RUN chmod +x /install.sh && \
  env ENVFILE=/env.sh /install.sh $ARCH && \
  rm -f /install.sh && \
  mkdir -p /target && \
  apt-get clean autoclean && \
  apt-get autoremove --yes && \
  rm -rf /var/cache/apt/archives && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /target
CMD ["/bin/bash"]

ENV CC_powerpc_unknown_linux_gnu='powerpc-linux-gnu-gcc'
ENV CXX_powerpc_unknown_linux_gnu='powerpc-linux-gnu-g++'
ENV AR_powerpc_unknown_linux_gnu='powerpc-linux-gnu-ar'
ENV CARGO_TARGET_POWERPC_UNKNOWN_LINUX_GNU_LINKER='powerpc-linux-gnu-gcc'
ENV CARGO_TARGET_POWERPC_UNKNOWN_LINUX_GNU_RUNNER='qemu-ppc -L /usr/powerpc-linux-gnu'
ENV CC_powerpc_unknown_linux_musl='powerpc-linux-musl-gcc'
ENV CXX_powerpc_unknown_linux_musl='powerpc-linux-musl-g++'
ENV AR_powerpc_unknown_linux_musl='powerpc-linux-musl-ar'
ENV CARGO_TARGET_POWERPC_UNKNOWN_LINUX_MUSL_LINKER='powerpc-linux-musl-gcc'
ENV CARGO_TARGET_POWERPC_UNKNOWN_LINUX_MUSL_RUNNER='qemu-ppc -L /usr/powerpc-linux-musl'
