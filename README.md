![ShadowMeter](shadowmeter-dark.png#gh-dark-mode-only)
![ShadowMeter](shadowmeter-light.png#gh-light-mode-only)

What happens when you implement a Network Anomaly Detection System (NADS) with [YAF](https://tools.netsa.cert.org/yaf/), [SuperMediator](https://tools.netsa.cert.org/super_mediator1/index.html), and an anomaly detection engine implemented with [PyTorch](https://www.pytorch.org/), and [Rust](https://www.rust-lang.org/)?  Well, let's prototype an experiment and find out.

# Motivation
This project is motivated by the following:
* Rust is an ideal language implementing microservices.
* Anomaly detection using PyTorch deep learning algorithmns is an effective way to detect network anomalies.
* YAF captures and generate advanced network flow data suitable for deep learning models.
* SuperMediator extracts advanced features from network flow records.

# Tentative schedule for implementing a microservices-based project
* Phase 1 - Build a Docker container with YAF, SuperMediator
* Phase 2 - Implement an anomaly detection engine with PyTorch using Rust
* Phase 3 - Integrate YAF, SuperMediator, and the anomaly detecion engine as microservices using Docker

# Project Blog
Subscibe to the project blog at [https://www.shadowmeter.io](https://www.shadowmeter.io)
###
Brand designed by [Glitschka Studios](https://www.glitschkastudios.com/)
# shadowmeter
