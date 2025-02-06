#!/usr/bin/env bash

CFLAGS="\
--target=wasm32-wasi \
--sysroot=$WASIX_SYSROOT \
-fPIC \
-nostartfiles"

LDFLAGS="\
-Wl,--no-entry \
-Wl,--import-memory \
-Wl,--export-all \
-Wl,--export-dynamic \
-Wl,--shared-memory \
-Xlinker --extra-features=bulk-memory,atomics \
-Wl,--shared"

$CC $CFLAGS $LDFLAGS -o side.wasm side.c

$WASMER -q run main.wasm --mapdir=/code:. >output

printf "handle: 0x5\nfoo = 42\n" | diff -u output - 1>/dev/null
