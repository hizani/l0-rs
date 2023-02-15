# l0-rust
Rewrote [WB internship task 0](https://github.com/hizani/l0) with Rust for fun.

The project uses Redis pub/sub system instead of NATS streaming (a.k.a STAN) because the official NATS lib for Rust lacks STAN support and I just want to try working with Redis.   
