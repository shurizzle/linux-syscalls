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

ENV CC_s390x_unknown_linux_gnu='s390x-linux-gnu-gcc'
ENV CXX_s390x_unknown_linux_gnu='s390x-linux-gnu-g++'
ENV AR_s390x_unknown_linux_gnu='s390x-linux-gnu-ar'
ENV CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_LINKER='s390x-linux-gnu-gcc'
ENV CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_RUNNER='qemu-s390x -L /usr/s390x-linux-gnu'
ENV CC_s390x_unknown_linux_musl='s390x-linux-musl-gcc'
ENV CXX_s390x_unknown_linux_musl='s390x-linux-musl-g++'
ENV AR_s390x_unknown_linux_musl='s390x-linux-musl-ar'
ENV CARGO_TARGET_S390X_UNKNOWN_LINUX_MUSL_LINKER='s390x-linux-musl-gcc'
ENV CARGO_TARGET_S390X_UNKNOWN_LINUX_MUSL_RUNNER='qemu-s390x -L /usr/s390x-linux-musl'
