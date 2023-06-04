FROM ubuntu:22.04
ARG ARCH
ENV ARCH="$ARCH"

ADD /scripts/install.sh /install.sh
ADD /docker/entrypoint.sh /entrypoint.sh

RUN chmod +x /install.sh && \
  env ENVFILE=/env.sh /install.sh $ARCH && \
  rm -f /install.sh && \
  mkdir -p /project && \
  apt-get clean autoclean && \
  apt-get autoremove --yes && \
  rm -rf /var/cache/apt/archives && \
  rm -rf /var/lib/apt/lists/* && \
  chmod +x /entrypoint.sh

WORKDIR /project
ENTRYPOINT ["/entrypoint.sh"]
cmd ["/bin/bash"]
