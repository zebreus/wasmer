use super::*;
use crate::syscalls::*;
use std::{fs::OpenOptions, io::BufReader};
use wasmer::{imports, TableType, Type};

/// ### `dl_load_symbol()`
/// Load a symbol from a dynamic library
#[instrument(level = "trace", skip_all, fields(name = field::Empty), ret)]
pub fn dl_load_symbol<M: MemorySize>(
    mut ctx: FunctionEnvMut<'_, WasiEnv>,
    symbol: WasmPtr<u8, M>,
    symbol_len: M::Offset,
    handle: WasmPtr<u32, M>,
    symbol_pointer: WasmPtr<u32, M>,
) -> Result<Errno, WasiError> {
    let env = ctx.data();
    let (memory, mut state) = unsafe { env.get_memory_and_wasi_state(&ctx, 0) };
    let symbol = unsafe { get_input_str_ok!(&memory, symbol, symbol_len) };

    Span::current().record("symbol", symbol.as_str());

    let symbol_value = wasi_try_ok!(dl_load_symbol_internal(&mut ctx, &symbol));

    let env = ctx.data();
    let (memory, mut state) = unsafe { env.get_memory_and_wasi_state(&ctx, 0) };

    wasi_try_mem_ok!(symbol_pointer.write(&memory, symbol_value));

    Ok(Errno::Success)
}

pub fn dl_load_symbol_internal(
    ctx: &mut FunctionEnvMut<'_, WasiEnv>,
    symbol: &str,
) -> Result<u32, Errno> {
    let (data, store) = ctx.data_and_store_mut();
    let symbol_value = data.experimental_dlsym(&symbol, store).unwrap();
    Ok(symbol_value)
}
