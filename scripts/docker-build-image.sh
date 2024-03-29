#!/bin/bash
VERSION="0.0.1"

docker build -t fidelismachine/shadowmeter:${VERSION} -t fidelismachine/shadowmeter:latest .
