#!/bin/bash

if [ ! -d /opt/shadowmeter/spool/yaf ]; then
    mkdir -p /opt/shadowmeter/spool/yaf
fi

if [ ! -z "${SHADOWMETER_PCAP_LIST}" ]; then
    SHADOWMETER_INPUT="--in ${SHADOWMETER_PCAP_LIST} --caplist"
    if [ ! -d /opt/shadowmeter/pcap ]; then
        mkdir -p /opt/shadowmeter/pcap
    fi
    echo "processing pcap list: ${SHADOWMETER_PCAP_LIST}"
elif [ ! -z "${SHADOWMETER_INTERFACE}" ]; then
    SHADOWMETER_INPUT="--in ${SHADOWMETER_INTERFACE} --live=pcap"
else
    echo "Missing environment variable SHADOWMETER_INTERFACE or SHADOWMETER_PCAP_LIST"
    exit 1
fi

if [ -z "${SHADOWMETER_OPTIONS}" ]; then
    SHADOWMETER_OPTIONS="--entropy --applabel --dpi --silk --verbose"
fi

export LTDL_LIBRARY_PATH=/opt/shadowmeter/lib/yaf

/opt/shadowmeter/bin/yaf ${SHADOWMETER_INPUT} \
    --max-payload=2048 --flow-stats --out /opt/shadowmeter/spool/yaf/yaf \
    --rotate 10 --lock ${SHADOWMETER_OPTIONS}

if [ ! -z "${SHADOWMETER_PCAP_LIST}" ]; then
    echo "finished processing pcap list: ${SHADOWMETER_PCAP_LIST}"
    # if in pcap procesing mode, then sleep until the service is explicity shut down
    while true
    do
        sleep 1
    done
fi
