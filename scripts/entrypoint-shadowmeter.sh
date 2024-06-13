#!/bin/bash

if [ ! -d  /var/shadowmeter/spool/processed ]; then
    mkdir /var/shadowmeter/spool/processed
fi

/opt/shadowmeter/bin/shadowmeter --input "/var/shadowmeter/spool/flow/flow*.json" \
    --output /var/shadowmeter/spool/processed \
    --database ${SHADOWMETER_DATABASE} \
    --geolite /var/shadowmeter/GeoLite2-ASN.mmdb \
    --sensor-id ${SHADOWMETER_ID}
