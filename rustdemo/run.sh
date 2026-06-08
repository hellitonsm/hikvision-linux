#!/bin/bash
DIR="$(cd "$(dirname "$0")" && pwd)"
export LD_LIBRARY_PATH="$DIR/lib:$DIR/lib/HCNetSDKCom"
exec "$DIR/target/debug/hcnetsdk-rust-demo" "$@"
