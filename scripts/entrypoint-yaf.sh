#!/bin/bash

if [ ! -d /opt/shadowmeter/spool/yaf ]; then
    mkdir -p /opt/shadowmeter/spool/yaf
fi

if [ -z "${YAF_OPTIONS}" ]; then
    YAF_OPTIONS="--entropy --applabel --dpi --silk --verbose"
fi

export LTDL_LIBRARY_PATH=/opt/shadowmeter/lib/yaf

/opt/shadowmeter/bin/yaf --in=${INTERFACE} --live=pcap \
    --max-payload=2048 --flow-stats --out /opt/shadowmeter/spool/yaf/yaf \
    --rotate 10 --lock --observation-domain ${OBSERVATION_DOMAIN} ${YAF_OPTIONS}
