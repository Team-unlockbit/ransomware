#!/bin/bash
# 암호화 프로그램 빌드
cargo build --release --bin encrypt

# 복호화 프로그램 빌드
cargo build --release --bin decrypt

#수기 작성 테스트 코드
#cargo build --release
#cargo run --release
