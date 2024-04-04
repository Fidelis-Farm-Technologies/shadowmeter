#!/bin/bash
VERSION=`git branch --show-current`

docker build -t fidelismachine/shadowmeter:${VERSION} -t fidelismachine/shadowmeter:latest .
