#!/bin/bash
DIR="$(cd "$(dirname "$0")" && pwd)"
export LD_LIBRARY_PATH="$DIR:$DIR/HCNetSDKCom"
exec "$DIR/QtClientDemo" "$@"
