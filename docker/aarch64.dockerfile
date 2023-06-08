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

ENV CC_aarch64_unknown_linux_gnu='aarch64-linux-gnu-gcc'
ENV CXX_aarch64_unknown_linux_gnu='aarch64-linux-gnu-g++'
ENV AR_aarch64_unknown_linux_gnu='aarch64-linux-gnu-ar'
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER='aarch64-linux-gnu-gcc'
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER='qemu-aarch64 -L /usr/aarch64-linux-gnu'
ENV CC_aarch64_unknown_linux_musl='/aarch64-musl.sh'
ENV CXX_aarch64_unknown_linux_musl='aarch64-linux-musl-g++'
ENV AR_aarch64_unknown_linux_musl='aarch64-linux-musl-ar'
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER='/aarch64-musl.sh'
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUNNER='qemu-aarch64 -L /usr/aarch64-linux-musl'
