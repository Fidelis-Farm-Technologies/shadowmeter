#!/bin/bash

if [ ! -d  /opt/shadowmeter/spool/processed ]; then
    mkdir -p  /opt/shadowmeter/spool/processed
fi

/opt/shadowmeter/bin/questdb_logger --input "/opt/shadowmeter/spool/flow/flow*.json" \
    --output /opt/shadowmeter/spool/processed \
    --questdb questdb:9009
