#!/bin/bash

/opt/shadowmeter/bin/sm_flow \
    --command export \
    --polling true \
    --input "/var/shadowmeter/spool/flow" \
    --format "questdb" \
    --uri ${SHADOWMETER_DB_URI} \
    --processed "/var/shadowmeter/spool/processed"
