#!/bin/bash

if [ ! -d /opt/shadowmeter/spool/yaf ]; then
    mkdir -p /opt/shadowmeter/spool/yaf
fi

if [ ! -z "${YAF_PCAP_LIST}" ]; then
    YAF_INPUT="--in ${YAF_PCAP_LIST} --caplist"
    if [ ! -d /opt/shadowmeter/pcap ]; then
        mkdir -p /opt/shadowmeter/pcap
    fi
    echo "processing pcap list: ${YAF_PCAP_LIST}"
elif [ ! -z "${YAF_INTERFACE}" ]; then
    YAF_INPUT="--in ${YAF_INTERFACE} --live=pcap"
else
    echo "Missing environment variable YAF_INTERFACE or YAF_PCAP_LIST"
    exit 1
fi

if [ -z "${YAF_OPTIONS}" ]; then
    YAF_OPTIONS="--entropy --applabel --dpi --silk --verbose"
fi

export LTDL_LIBRARY_PATH=/opt/shadowmeter/lib/yaf

/opt/shadowmeter/bin/yaf ${YAF_INPUT} \
    --max-payload=2048 --flow-stats --out /opt/shadowmeter/spool/yaf/yaf \
    --rotate 10 --lock ${YAF_OPTIONS}

if [ ! -z "${YAF_PCAP_LIST}" ]; then
    echo "finished processing pcap list: ${YAF_PCAP_LIST}"
    # if in pcap procesing mode, then sleep until the service is explicity shut down
    while true
    do
        sleep 1
    done
fi
