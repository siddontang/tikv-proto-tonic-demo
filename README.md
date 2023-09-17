# TiKV gRPC protocols based on tonic

This is just a demo to support [TiKV](https://github.com/tikv/tikv) gRPC protocols based on [tonic](https://github.com/hyperium/tonic). 

```rust
cargo run --bin server
```

In another terminal:

```bash
# Use grpcurl to list services
grpcurl -plaintext '[::1]:50051' list

# Use grpcui for visual interactions on the web
grpcui -plaintext '[::1]:50051' 
```