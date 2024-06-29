#!/bin/bash

CONFIG_FILE=/opt/shadowmeter/etc/super_mediator.conf

if [ "$SHADOWMETER_EXPORT_MODE" == "CACHE" ]; then
    CONFIG_FILE=/opt/shadowmeter/etc/super_mediator_cache.conf
    if [ ! -d /var/shadowmeter/spool/dpi ]; then
        mkdir /var/shadowmeter/spool/dpi
    fi
    if [ ! -d /var/shadowmeter/spool/dns ]; then
        mkdir /var/shadowmeter/spool/dns
    fi
    if [ ! -d /var/shadowmeter/spool/tls ]; then
        mkdir /var/shadowmeter/spool/tls
    fi
fi

if [ ! -d /var/shadowmeter/spool/yaf ]; then
    mkdir /var/shadowmeter/spool/yaf
else
    rm /var/shadowmeter/spool/yaf/yaf-*.lock
fi

if [ ! -d /var/shadowmeter/spool/flow ]; then
    mkdir /var/shadowmeter/spool/flow
else
    rm /var/shadowmeter/spool/flow/.flow.*
fi

if [ ! -d /var/shadowmeter/spool/processed ]; then
    mkdir /var/shadowmeter/spool/processed
fi

/opt/shadowmeter/bin/super_mediator --config ${CONFIG_FILE}


