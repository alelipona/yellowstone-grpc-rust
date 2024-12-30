# yellowstone-grpc-rust

Yellowstone gRPC 是获取 Solana 链上数据最快的方式。数据以流的方式推送，客户端需要配置订阅来获取和解析数据。

本教程旨在提供一些简单的订阅配置例子，帮助你快速熟悉此工具。

---

## subscribe-tx 订阅账户交易

``` bash
# cargo test --package yellowstone-grpc-rust --bin yellowstone-grpc-rust -- subscribe_tx::subscribe_tx_tests::test_subscribe_tx --exact --show-output
```
