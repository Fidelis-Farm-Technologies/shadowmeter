![ShadowMeter](shadowmeter-dark.png#gh-dark-mode-only)
![ShadowMeter](shadowmeter-light.png#gh-light-mode-only)

What happens when you implement a Network Anomaly Detection System (NADS) with [YAF](https://tools.netsa.cert.org/yaf/), [SuperMediator](https://tools.netsa.cert.org/super_mediator1/index.html), and an anomaly detection engine implemented with [PyTorch](https://www.pytorch.org/), and [Rust](https://www.rust-lang.org/)?  Well, let's prototype an experiment and find out.

## Motivation
This project is motivated by the following:
* Rust is an ideal language implementing microservices.
* Anomaly detection using PyTorch deep learning algorithmns is an effective way to detect network anomalies.
* YAF captures and generate advanced network flow data suitable for deep learning models.
* SuperMediator extracts advanced features from network flow records.

## Project Schedule
- [&check;] Phase 1 - Build a Docker container with YAF, SuperMediator
- [ ] Phase 2 - Implement an anomaly detection (AD) engine with PyTorch using Rust
- [ ] Phase 3 - Integrate YAF, SuperMediator, and the AD engine as microservices

## Docker Image

````
docker pull fidelismachine/shadowmeter:latest
````
---
## Community
[Blog](https://www.shadowmeter.io)



&copy;2024 Fidelis Farm & Technologies, LLC.
