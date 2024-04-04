#!/bin/bash
VERSION=`git branch --show-current`

docker push fidelismachine/shadowmeter:${VERSION} 
docker push fidelismachine/shadowmeter:latest 
