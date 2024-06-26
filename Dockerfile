
# ---------------------------------------------------------------
#
# ---------------------------------------------------------------
FROM shadowmeter_builder AS builder
FROM bitnami/minideb:bookworm AS runner


# ---------------------------------------------------------------
#
# ---------------------------------------------------------------

WORKDIR /opt/shadowmeter
RUN mkdir -p /opt/shadowmeter/scripts /opt/shadowmeter/etc \
    /opt/shadowmeter/lib/pytorch

COPY --from=builder /opt/shadowmeter /opt/shadowmeter
COPY --from=builder /builder/shadowmeter_engine/target/release/shadowmeter /opt/shadowmeter/bin
COPY --from=builder /base/libtorch/lib/* /opt/shadowmeter/lib/pytorch/

COPY ./scripts/entrypoint-yaf.sh /opt/shadowmeter/scripts
COPY ./scripts/entrypoint-super_mediator.sh /opt/shadowmeter/scripts
COPY ./scripts/entrypoint-shadowmeter.sh /opt/shadowmeter/scripts

COPY ./etc/yaf/super_mediator.conf /opt/shadowmeter/etc
COPY ./etc/yaf/super_mediator_cache.conf /opt/shadowmeter/etc
COPY ./etc/yaf/yafDPIRules.conf /opt/shadowmeter/etc
COPY ./etc/logrotate/shadowmeter.logrotate /etc/logrotate.d/shadowmeter


COPY --from=builder \
    /lib/libndpi.so.4 \
    /usr/lib/x86_64-linux-gnu/libpcap.so.1.10.3 \
    /usr/lib/x86_64-linux-gnu/libglib-2.0.so.0 \
    /usr/lib/x86_64-linux-gnu/libpcre2-8.so.0.11.2 \
    /usr/lib/x86_64-linux-gnu/libpcre.so.3.13.3 \
    /usr/lib/x86_64-linux-gnu/libdbus-1.so.3.32.4 \
    /usr/lib/x86_64-linux-gnu/libcrypto.so.3 \
    /usr/lib/x86_64-linux-gnu/

RUN echo "/opt/shadowmeter/lib" > /etc/ld.so.conf.d/shadowmeter.conf
RUN echo "/opt/shadowmeter/lib/yaf" > /etc/ld.so.conf.d/yaf.conf
RUN echo "/opt/shadowmeter/lib/pytorch" > /etc/ld.so.conf.d/pytorch.conf
RUN ldconfig

VOLUME /opt/shadowmeter/spool
