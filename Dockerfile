FROM bitnami/minideb:bookworm as builder
RUN apt-get update
RUN apt-get install -y build-essential xsltproc libglib2.0-dev \
    autotools-dev automake libpcap-dev libpcre3-dev libssl-dev
WORKDIR /builder
COPY . .
#
# build libfixbuf
#
WORKDIR /builder/cert-nsa-libfixbuf
RUN ./configure --prefix=/opt/shadowmeter --disable-tools 
RUN make && make install
#
# Update the LD_LIBRARY_PATH
#
RUN echo "/opt/shadowmeter/lib" > /etc/ld.so.conf.d/shadowmeter.conf
RUN ldconfig
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
WORKDIR /builder/cert-super_mediator
RUN ./configure --prefix=/opt/shadowmeter 
RUN make && make install


# ---------------------------------------------------------------
#
# ---------------------------------------------------------------
FROM bitnami/minideb:bookworm AS runner
COPY --from=builder /opt/shadowmeter /opt/shadowmeter

COPY --from=builder \
    /usr/lib/x86_64-linux-gnu/libpcap.so.1.10.3 \
    /usr/lib/x86_64-linux-gnu/libglib-2.0.so.0 \
    /usr/lib/x86_64-linux-gnu/libpcre2-8.so.0.11.2 \
    /usr/lib/x86_64-linux-gnu/libpcre.so.3.13.3 \
    /usr/lib/x86_64-linux-gnu/libdbus-1.so.3.32.4 \
    /usr/lib/x86_64-linux-gnu/

COPY --from=builder /etc/ld.so.conf.d/shadowmeter.conf /etc/ld.so.conf.d/
RUN ldconfig
RUN mkdir -p /opt/shadowmeter/log /opt/shadowmeter/run /opt/shadowmeter/spool

#COPY /shadowmeter.logrotate /etc/logrotate.d/shadowmeter

VOLUME /opt/shadowmeter/log
VOLUME /opt/shadowmeter/spool
VOLUME /opt/shadowmeter/run
VOLUME /opt/shadowmeter/etc

#COPY /docker-entrypoint.sh /
#ENTRYPOINT ["/docker-entrypoint.sh"]
