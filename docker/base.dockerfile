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
