#!/bin/bash
VERSION=`git branch --show-current`

docker build --no-cache -t fidelismachine/shadowmeter_nginx:${VERSION} -t fidelismachine/shadowmeter_nginx:latest .
