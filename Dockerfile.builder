FROM shadowmeter_base AS base
# ---------------------------------------------------------------
#
# ---------------------------------------------------------------
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
RUN git checkout ndpi-4.8
RUN ./configure --prefix=/opt/shadowmeter --enable-entropy --enable-applabel --enable-dpi --with-ndpi 
RUN make && make install
#
# build super_mediator
#
WORKDIR /builder/cert-nsa-super_mediator
RUN git checkout ndpi-4.8
RUN ./configure --prefix=/opt/shadowmeter LIBS=-lndpi
RUN make && make install
#
# build shadowmeter_engine
#
ENV LIBTORCH=/base/libtorch
ENV LIBTORCH_INCLUDE=/base/libtorch
ENV LIBTORCH_LIB=/base/libtorch
ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
ENV LIBTORCH_BYPASS_VERSION_CHECK=1
WORKDIR /builder/shadowmeter_engine
RUN cargo build --release



