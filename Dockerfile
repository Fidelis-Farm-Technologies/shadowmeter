FROM bitnami/minideb:bookworm as builder
RUN apt-get update
RUN apt-get install -y build-essential xsltproc libglib2.0-dev \
    autotools-dev automake libpcap-dev libpcre3-dev libssl-dev
WORKDIR /builder
COPY . .
#
# Update the LD_LIBRARY_PATH
#
RUN echo "/opt/shadowmeter/lib" > /etc/ld.so.conf.d/shadowmeter.conf
RUN ldconfig
#
# build libfixbuf
#
WORKDIR /builder/cert-nsa-libfixbuf
RUN ./configure --prefix=/opt/shadowmeter --disable-tools 
RUN make && make install
#
# build yaf
#
WORKDIR /builder/cert-nsa-yaf
#./configure --prefix=/opt/shadowmeter --with-ndpi  --enable-entropy --enable-applabel --enable-dpi
RUN ./configure --prefix=/opt/shadowmeter --enable-entropy --enable-applabel --enable-dpi 
RUN make && make install
#
# build super_mediator
#
WORKDIR /builder/cert-nsa-super_mediator
RUN ./configure --prefix=/opt/shadowmeter 
RUN make && make install

# ---------------------------------------------------------------
#
# ---------------------------------------------------------------
FROM bitnami/minideb:bookworm AS runner
COPY --from=builder /opt/shadowmeter /opt/shadowmeter
RUN mkdir -p /opt/shadowmeter/log /opt/shadowmeter/spool \
    /opt/shadowmeter/scripts /opt/shadowmeter/etc

COPY --from=builder \
    /usr/lib/x86_64-linux-gnu/libpcap.so.1.10.3 \
    /usr/lib/x86_64-linux-gnu/libglib-2.0.so.0 \
    /usr/lib/x86_64-linux-gnu/libpcre2-8.so.0.11.2 \
    /usr/lib/x86_64-linux-gnu/libpcre.so.3.13.3 \
    /usr/lib/x86_64-linux-gnu/libdbus-1.so.3.32.4 \
    /usr/lib/x86_64-linux-gnu/

COPY --from=builder /builder/scripts/yaf-entrypoint.sh /opt/shadowmeter/scripts
COPY --from=builder /builder/scripts/super_mediator-entrypoint.sh /opt/shadowmeter/scripts
COPY --from=builder /builder/etc/super_mediator.conf /opt/shadowmeter/etc
COPY --from=builder /builder/etc/super_mediator_text.conf /opt/shadowmeter/etc
COPY --from=builder /builder/etc/yafDPIRules.conf /opt/shadowmeter/etc
COPY --from=builder /builder/etc/shadowmeter.logrotate /opt/shadowmeter/etc
RUN echo "/opt/shadowmeter/lib" > /etc/ld.so.conf.d/shadowmeter.conf
RUN echo "/opt/shadowmeter/lib/yaf" >> /etc/ld.so.conf.d/shadowmeter.conf
RUN ldconfig

VOLUME /opt/shadowmeter/log
VOLUME /opt/shadowmeter/spool


