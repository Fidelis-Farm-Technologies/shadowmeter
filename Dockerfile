
# ---------------------------------------------------------------
#
# ---------------------------------------------------------------
FROM shadowmeter_base AS builder
FROM bitnami/minideb:bookworm AS runner

# ---------------------------------------------------------------
#
# ---------------------------------------------------------------

COPY --from=builder /opt/shadowmeter /opt/shadowmeter
RUN mkdir -p /opt/shadowmeter/scripts /opt/shadowmeter/etc
WORKDIR /opt/shadowmeter
COPY ./scripts/entrypoint-yaf.sh /opt/shadowmeter/scripts
COPY ./scripts/entrypoint-super_mediator.sh /opt/shadowmeter/scripts
COPY ./scripts/entrypoint-shadowmeter_logger.sh /opt/shadowmeter/scripts

COPY --from=builder \
    /usr/lib/x86_64-linux-gnu/libpcap.so.1.10.3 \
    /usr/lib/x86_64-linux-gnu/libglib-2.0.so.0 \
    /usr/lib/x86_64-linux-gnu/libpcre2-8.so.0.11.2 \
    /usr/lib/x86_64-linux-gnu/libpcre.so.3.13.3 \
    /usr/lib/x86_64-linux-gnu/libdbus-1.so.3.32.4 \
    /usr/lib/x86_64-linux-gnu/libcrypto.so.3 \
    /usr/lib/x86_64-linux-gnu/

COPY --from=builder /builder/shadowmeter_logger/target/release/shadowmeter_logger /opt/shadowmeter/bin
COPY --from=builder /builder/etc/super_mediator.conf /opt/shadowmeter/etc
COPY --from=builder /builder/etc/super_mediator_cache.conf /opt/shadowmeter/etc
COPY --from=builder /builder/etc/yafDPIRules.conf /opt/shadowmeter/etc
COPY --from=builder /builder/etc/shadowmeter.logrotate /opt/shadowmeter/etc
RUN echo "/opt/shadowmeter/lib" > /etc/ld.so.conf.d/shadowmeter.conf
RUN echo "/opt/shadowmeter/lib/yaf" >> /etc/ld.so.conf.d/shadowmeter.conf
RUN ldconfig

VOLUME /opt/shadowmeter/spool
