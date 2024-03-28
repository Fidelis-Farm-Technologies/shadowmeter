#!/bin/bash
INTERFACE=enp4s0

/opt/shadowmeter/bin/yaf --in=${INTERFACE} --live=pcap --verbose --log /var/log/yaf.log --entropy --applabel --ndpi --flow-stats --max-payload=2048 --out 127.0.0.1 --ipfix tcp --ipfix-port 18000 --daemonize




