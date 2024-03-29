#!/bin/bash

CONFIG_FILE=/opt/shadowmeter/etc/super_mediator.conf

[ "$OUTPUT_MODE" == "TEXT" ] && CONFIG_FILE=/opt/shadowmeter/etc/super_mediator_text.conf


if [ ! -d /opt/shadowmeter/spool/dpi ]; then
    mkdir -p /opt/shadowmeter/spool/dpi
fi
if [ ! -d /opt/shadowmeter/spool/dns ]; then
    mkdir -p /opt/shadowmeter/spool/dns
fi
if [ ! -d /opt/shadowmeter/spool/tls ]; then
    mkdir -p /opt/shadowmeter/spool/tls
fi
if [ ! -d /opt/shadowmeter/spool/flow ]; then
    mkdir -p /opt/shadowmeter/spool/flow
fi

/opt/shadowmeter/bin/super_mediator --config ${CONFIG_FILE}


