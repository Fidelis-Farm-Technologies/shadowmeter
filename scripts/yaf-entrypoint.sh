#!/bin/bash

/opt/shadowmeter/bin/yaf --in=${INTERFACE} --live=pcap \
    --max-payload=2048 --out /opt/shadowmeter/spool/yaf/yaf \
    --rotate 10 --lock --entropy --applabel --flow-stats \
    --verbose 
    


