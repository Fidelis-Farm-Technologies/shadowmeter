#!/bin/bash

if [ ! -d /opt/shadowmeter/spool/yaf ]; then
    mkdir -p /opt/shadowmeter/spool/yaf
fi

export LTDL_LIBRARY_PATH=/opt/shadowmeter/lib/yaf

/opt/shadowmeter/bin/yaf --in=${INTERFACE} --live=pcap \
    --max-payload=2048 --out /opt/shadowmeter/spool/yaf/yaf \
    --rotate 10 --lock --flow-stats --verbose \
    --entropy --applabel --dpi


