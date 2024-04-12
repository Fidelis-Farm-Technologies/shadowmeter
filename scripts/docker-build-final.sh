#!/bin/bash
VERSION=`git branch --show-current`

docker build --no-cache -t fidelismachine/shadowmeter:${VERSION} -t fidelismachine/shadowmeter:latest .
