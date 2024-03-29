#!/bin/bash

docker tag fidelismachine/shadowmeter fidelismachine/shadowmeter:beta
docker tag fidelismachine/shadowmeter fidelismachine/shadowmeter:latest
docker push fidelismachine/shadowmeter
