#!/bin/bash

if [ ! -d  /opt/shadowmeter/spool/processed ]; then
    mkdir -p  /opt/shadowmeter/spool/processed
fi

/opt/shadowmeter/bin/shadowmeter_logger --input "/opt/shadowmeter/spool/flow/flow*.json" \
    --output /opt/shadowmeter/spool/processed \
    --questdb shadowdb:9009 \
    --sensor-id ${SENSOR_ID}
