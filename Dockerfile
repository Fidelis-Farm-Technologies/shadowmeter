
# ---------------------------------------------------------------
#
# ---------------------------------------------------------------
FROM shadowmeter_apps AS builder
FROM bitnami/minideb:bookworm AS runner

# ---------------------------------------------------------------
#
# ---------------------------------------------------------------

WORKDIR /opt/shadowmeter
RUN mkdir -p /opt/shadowmeter/ \
             /opt/shadowmeter/scripts /opt/shadowmeter/etc \
             /opt/shadowmeter/lib /opt/shadowmeter/lib/pytorch

COPY --from=builder /usr/local/lib /opt/shadowmeter/lib
COPY --from=builder /base/libtorch/lib /opt/shadowmeter/lib/pytorch

COPY --from=builder /builder/sm_flow/target/release/sm_flow /opt/shadowmeter/bin/sm_flow
COPY --from=builder /builder/sm_detect/target/release/sm_detect /opt/shadowmeter/bin/sm_detect
COPY --from=builder /usr/local/bin/yaf /opt/shadowmeter/bin/yaf
COPY --from=builder /usr/local/bin/duckdb /opt/shadowmeter/bin/duckdb

COPY ./scripts/entrypoint-sm_yaf.sh /opt/shadowmeter/scripts/
COPY ./scripts/entrypoint-sm_import.sh /opt/shadowmeter/scripts/
COPY ./scripts/entrypoint-sm_export.sh /opt/shadowmeter/scripts/
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
