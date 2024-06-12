#!/bin/bash

if [ ! -d  /var/shadowmeter/spool/yaf ]; then
    mkdir /var/shadowmeter/spool/yaf
fi

if [ ! -z "${SHADOWMETER_PCAP_LIST}" ]; then
    SHADOWMETER_INPUT="--in ${SHADOWMETER_PCAP_LIST} --caplist"
    echo "pcap offline: ${SHADOWMETER_INPUT}"
elif [ ! -z "${SHADOWMETER_INTERFACE}" ]; then
    SHADOWMETER_INPUT="--in ${SHADOWMETER_INTERFACE} --live=pcap"
     echo "pcap live: ${SHADOWMETER_INPUT}"
else
    echo "Missing environment variable SHADOWMETER_INTERFACE or SHADOWMETER_PCAP_LIST"
    exit 1
fi

if [ -z "${SHADOWMETER_OPTIONS}" ]; then
    # SHADOWMETER_OPTIONS="--entropy --applabel --dpi --silk --verbose --max-payload=2048 --flow-stats --out /var/shadowmeter/spool/yaf/yaf --lock"
    SHADOWMETER_OPTIONS="--entropy --ndpi --verbose --max-payload=2048 --flow-stats --out /var/shadowmeter/spool/yaf/yaf --lock"
fi

export LTDL_LIBRARY_PATH=/opt/shadowmeter/lib/yaf

if [ ! -z "${SHADOWMETER_PCAP_LIST}" ]; then
    /opt/shadowmeter/bin/yaf ${SHADOWMETER_INPUT} ${SHADOWMETER_OPTIONS}

    echo "finished processing pcap list: ${SHADOWMETER_PCAP_LIST}"
    # if in pcap procesing mode, then sleep until the service is explicity shut down
    while true
    do
        sleep 1
    done
else
    /opt/shadowmeter/bin/yaf ${SHADOWMETER_INPUT} ${SHADOWMETER_OPTIONS} --rotate 10 
fi
