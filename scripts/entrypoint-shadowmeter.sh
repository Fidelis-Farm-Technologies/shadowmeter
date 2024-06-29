#!/bin/bash
SHADOWMETER_GEO_OPTIONS=
SHADOWMETER_GEO_ASN=/var/shadowmeter/maxmind/GeoLite2-ASN.mmdb
SHADOWMETER_GEO_COUNTRY=/var/shadowmeter/maxmind/GeoLite2-Country.mmdb

if [ ! -d  /var/shadowmeter/spool/processed ]; then
    mkdir /var/shadowmeter/spool/processed
fi

if [ -f /var/shadowmeter/maxmind/GeoLite2-ASN.mmdb ]; then
  SHADOWMETER_GEO_OPTIONS="--geolite-asn ${SHADOWMETER_GEO_ASN}"
fi

if [ -f /var/shadowmeter/maxmind/GeoLite2-ASN.mmdb ]; then
   SHADOWMETER_GEO_OPTIONS="${SHADOWMETER_GEO_OPTIONS} --geolite-country ${SHADOWMETER_GEO_COUNTRY}"
fi

/opt/shadowmeter/bin/shadowmeter --input "/var/shadowmeter/spool/flow/flow*.json" \
    --sensor-id ${SHADOWMETER_ID} \
    --output /var/shadowmeter/spool/processed \
    --database ${SHADOWMETER_DATABASE} \
    ${SHADOWMETER_GEO_OPTIONS}

