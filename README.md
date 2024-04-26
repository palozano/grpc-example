To run the server `cargo run --bin server`, and to send from the client `cargo run --bin client`. 

You can send gRPC requests from the terminal if you want: `$ grpcurl -plaintext -proto ./proto/calculator.proto -d '{"x": 1, "y": 2} '[::1]:50051' calculator.Calculator.Add`, and if the server is running, receive the result.

Go inside `src/client.rs` to change what it does.

Definitions of the protobufs, inside `proto/`. 
