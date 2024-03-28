#!/bin/bash

#
# build libfixbuf
#
cd cert-nsa-libfixbuf
./configure --prefix=/opt/shadowmeter
make & make install
cd ..
#
# build yaf
#
cd cert-nsa-yaf
./configure --prefix=/opt/shadowmeter --with-ndpi  --enable-entropy --enable-applabel --enable-dpi
make & make install
cd ..
#
# build super_mediator
#
cd cert-nsa-super_mediator
./configure --prefix=/opt/shadowmeter
make & make install
cd ..

