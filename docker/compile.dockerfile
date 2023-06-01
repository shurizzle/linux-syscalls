FROM ubuntu:22.04
ARG ARCH
ENV ARCH="$ARCH"

ADD /scripts/install.sh /install.sh
ADD /scripts/compile.sh /compile.sh

RUN chmod +x /install.sh && \
  chmod +x /compile.sh && \
  /install.sh $ARCH && \
  rm -f /install.sh && \
  mkdir -p /project && \
  apt-get clean autoclean && \
  apt-get autoremove --yes && \
  rm -rf /var/cache/apt/archives && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /project
ENTRYPOINT ["/compile.sh"]
