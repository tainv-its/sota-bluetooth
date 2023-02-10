

FROM alpine:3.15

ENV CRYPTOGRAPHY_DONT_BUILD_RUST=1

ADD repositories /etc/apk/repositories
RUN apk update && \
    apk -U add alpine-sdk && \
    apk add pkgconfig dbus-dev

RUN mkdir -p /var/cache/distfiles && \
    adduser -D packager && \
    addgroup packager abuild && \
    chgrp abuild /var/cache/distfiles && \
    chmod g+w /var/cache/distfiles && \
    echo "packager    ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

ADD setup.sh /home/packager/bin/setup.sh
# TODO: customize /etc/abuild.conf

WORKDIR /work
USER    packager
