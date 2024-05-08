![ShadowMeter](shadowmeter-dark.png#gh-dark-mode-only)
![ShadowMeter](shadowmeter-light.png#gh-light-mode-only)


An Network Anomaly Detection System designed for IoT /OT and built with the following microservices:
- [PyTorch](https://www.pytorch.org/)-based anomaly detection engine implemented with [Rust](https://www.rust-lang.org/).
- [YAF](https://tools.netsa.cert.org/yaf/)
- [SuperMediator](https://tools.netsa.cert.org/super_mediator1/index.html)
- [QuestDB](https://questdb.io/download/)
- [Grafana](https://grafana.com/oss/grafana/)

## Motivation
This project is motivated by the following:
* Rust is an ideal language implementing microservices.
* Anomaly detection using PyTorch deep learning algorithmns is an effective way to detect network anomalies.
* YAF captures and generate advanced network flow data suitable for deep learning models.
* SuperMediator extracts advanced features from network flow records.

## Project Schedule
- [&check;] Phase 1 - Build a Docker container with YAF, SuperMediator
- [&check;] Phase 2 - Publish docker-compose (microservices) integrating YAF, SuperMediator, ShadowMeter, QuestDB, and Grafana
- [&nbsp;] Phase 3 - Add autoencoder for anomaly detection using the PyTorch library

## Docker Image

[DockerHub](https://hub.docker.com/r/fidelismachine/shadowmeter)

````
docker pull fidelismachine/shadowmeter:latest
````
---
## Quick Start
Although the pytorch functionality (phase 3) is not implemented yet, you can still run ShadowMeter as a network monitoring probe.  First, create an .env file with the following settings:
```
SHADOWMETER_INTERFACE="enp4s0"
SHADOWMETER_ID="sm1"
SHADOWMETER_USERNAME=admin
SHADOWMETER_PASSWORD="xyz"
```
- Set SHADOWMETER_INTERFACE to the monitoring network interface
- Set SHADOWMETER_ID to an arbitrary unique sensor id label that will appear in the database
- Set SHADOWMETER_USERNAME and SHADOWMETER_PASSWORD to the QuestDB username and password, respectively

Finally, review then run Docker Compose with the docker-compose.yml file included in this repository.
```
# docker-compose up -d
```

## Community

For more information join the community at our [blog](https://www.shadowmeter.io).



&copy;2024 Fidelis Farm & Technologies, LLC.
