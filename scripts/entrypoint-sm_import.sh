#!/bin/bash
SHADOWMETER_GEO_OPTIONS=
SHADOWMETER_GEO_ASN=/var/maxmind/GeoLite2-ASN.mmdb
SHADOWMETER_GEO_COUNTRY=/var/maxmind/GeoLite2-Country.mmdb

if [ ! -d  /var/shadowmeter/spool/processed ]; then
    mkdir /var/shadowmeter/spool/processed
fi

if [ ! -d  /var/shadowmeter/spool/flow ]; then
    mkdir /var/shadowmeter/spool/flow
fi

if [ -f ${SHADOWMETER_GEO_ASN} ]; then
  SHADOWMETER_GEO_OPTIONS="--asn ${SHADOWMETER_GEO_ASN}"
fi

if [ -f ${SHADOWMETER_GEO_COUNTRY} ]; then
   SHADOWMETER_GEO_OPTIONS="${SHADOWMETER_GEO_OPTIONS} --country ${SHADOWMETER_GEO_COUNTRY}"
fi

/opt/shadowmeter/bin/sm_flow \
    --command import \
    --polling true \
    --input "/var/shadowmeter/spool/yaf" \
    --output "/var/shadowmeter/spool/flow" \
    --processed "/var/shadowmeter/spool/processed" \
    --observation ${SHADOWMETER_OBSERVATION_ID} \
    ${SHADOWMETER_GEO_OPTIONS}
    

