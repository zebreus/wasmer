#!/usr/bin/env bash

CFLAGS="\
--target=wasm32-wasi \
--sysroot=$WASIX_SYSROOT \
-fPIC"

LDFLAGS="\
-Wl,--no-entry \
-nostartfiles \
-Wl,--import-memory \
-Wl,--shared-memory \
-Wl,--max-memory=4294967296 \
-Wl,--export-dynamic \
-Wl,--export=__heap_base \
-Wl,--export=__stack_pointer \
-Wl,--export=__data_end \
-Wl,--export=__wasm_init_tls \
-Wl,--export=__wasm_signal \
-Wl,--export=__tls_size \
-Wl,--export=__tls_align \
-Wl,--export=__tls_base \
-lwasi-emulated-mman"

$CC $CFLAGS $LDFLAGS -o side.wasm side.c

$WASMER -q run main.wasm --mapdir=/code:. >output

printf "0" | diff -u output - 1>/dev/null
