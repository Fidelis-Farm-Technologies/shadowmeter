#!/bin/bash
VERSION=`git branch --show-current`

docker build --no-cache -t fidelismachine/shadowmeter:${VERSION} -t fidelismachine/shadowmeter:latest .
# docker build -t fidelismachine/shadowmeter:${VERSION} -t fidelismachine/shadowmeter:latest .