#!/bin/bash
VERSION=`git branch --show-current`

docker push fidelismachine/shadowmeter_nginx:${VERSION} 
docker push fidelismachine/shadowmeter_nginx:latest 
