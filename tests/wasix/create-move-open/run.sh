#!/usr/bin/env bash

$WASMER -q run main.wasm --dir . >output

rm -rf test1 test2 2>/dev/null && printf "0" | diff -u output - 1>/dev/null
