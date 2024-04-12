#!/bin/bash

if [ ! -d  /opt/shadowmeter/spool/processed ]; then
    mkdir -p  /opt/shadowmeter/spool/processed
fi

/opt/shadowmeter/bin/shadowmeter --input "/opt/shadowmeter/spool/flow/flow*.json" \
    --output /opt/shadowmeter/spool/processed \
    --database ${DATABASE} \
    --sensor-id ${SENSOR_ID}
