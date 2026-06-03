#!/bin/bash
DIR="$(cd "$(dirname "$0")" && pwd)"
export LD_LIBRARY_PATH="$DIR/../Linux64/lib:$DIR/../Linux64/lib/HCNetSDKCom"
exec "$DIR/target/debug/hcnetsdk-rust-demo" "$@"
