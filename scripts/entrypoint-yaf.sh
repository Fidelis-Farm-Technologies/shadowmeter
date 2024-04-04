#!/bin/bash

if [ ! -d /opt/shadowmeter/spool/yaf ]; then
    mkdir -p /opt/shadowmeter/spool/yaf
fi

if [ -z "${YAF_OBSERVATION_DOMAIN_ID}" ]; then
    YAF_OBSERVATION_DOMAIN_ID=3
fi

if [ ! -z "${YAF_PCAPS}" ]; then
    YAF_INPUT="--in ${YAF_PCAPS} --caplist"
    if [ ! -d /opt/shadowmeter/pcap ]; then
        mkdir -p /opt/shadowmeter/pcap
    fi
    echo "processing pcap list: ${YAF_PCAPS}"
elif [ ! -z "${YAF_INTERFACE}" ]; then
    YAF_INPUT="--in ${YAF_INTERFACE} --live=pcap"
else
    echo "Missing environment variable YAF_INTERFACE or YAF_PCAPS"
    exit 1
fi

if [ -z "${YAF_OPTIONS}" ]; then
    YAF_OPTIONS="--entropy --applabel --dpi --silk --verbose"
fi

export LTDL_LIBRARY_PATH=/opt/shadowmeter/lib/yaf

/opt/shadowmeter/bin/yaf ${YAF_INPUT} \
    --max-payload=2048 --flow-stats --out /opt/shadowmeter/spool/yaf/yaf \
    --rotate 10 --lock --observation-domain ${YAF_OBSERVATION_DOMAIN_ID} ${YAF_OPTIONS}

if [ ! -z "${YAF_PCAPS}" ]; then
    echo "finished processing pcap list: ${YAF_PCAPS}"
    # if in pcap procesing mode, then sleep until the service is explicity shut down
    while true
    do
        sleep 1
    done
fi
