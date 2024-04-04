#!/bin/bash
VERSION="0.0.2"

docker build -t fidelismachine/shadowmeter:${VERSION} -t fidelismachine/shadowmeter:latest .
