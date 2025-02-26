Привет! Конечно, вот перевод на английский:

---

# yellowstone-grpc-rust

Yellowstone gRPC is the fastest way to fetch data from the Solana blockchain. Data is streamed, and clients need to configure subscriptions to receive and parse the data.

This tutorial aims to provide some simple subscription configuration examples to help you quickly get familiar with this tool.

---

## subscribe-tx: Subscribe to Account Transactions

```bash
# cargo test --package yellowstone-grpc-rust --bin yellowstone-grpc-rust -- subscribe_tx::subscribe_tx_tests::test_subscribe_tx --exact --show-output
```

## subscribe-logs: Subscribe to Token Transactions and Parse Logs to Include the Latest Data of the Pool

```bash
# cargo test --package yellowstone-grpc-rust --bin yellowstone-grpc-rust -- subscribe_logs::subscribe_tx_tests --show-output
```

### PUMP Pool

```rust
PumpEvent Some(
    TradeEvent {
        mint: Aa4QWNkS3RLUv7DA9BM1a2Hzm4HDQo5PyRefqDJnpump,
        sol_amount: 114631249,
        token_amount: 606908476440,
        is_buy: false,
        user: 2i1S6JReBVBtHRS3WnZjzrf6EPqgMWrtWkUC4T68Y1Yq,
        timestamp: 1735823228,
        virtual_sol_reserves: 77916811328,
        virtual_token_reserves: 413132930150753,
        real_sol_reserves: 47916811328,
        real_token_reserves: 133232930150753,
    },
)
```

### RAYDIUM

```rust
RaydiumEvent Some(
    SwapBaseInLog {
        log_type: 3,
        amount_in: 30000000,
        minimum_out: 258187550602,
        direction: 2,
        user_source: 30000000,
        pool_coin: 43626306718,
        pool_pc: 391838738896727,
        out_amount: 268593366858,
    },
)
```

---
