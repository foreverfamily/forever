#!/usr/bin/env bash
#libprotoc 3.5.1

protoc --rust_out=. *.proto
protoc --rust-grpc_out=. *.proto