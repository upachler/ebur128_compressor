#!/bin/bash
#


BASE_NAME=signal_generator
BINARY=target/debug/lib$BASE_NAME.dylib
TARGET=~/.lv2/signal-generator.lv2/


cargo build
mkdir -p $TARGET
cp manifest.ttl $BASE_NAME.ttl $BINARY $TARGET


