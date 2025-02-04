use std::{fs::OpenOptions, io::BufReader};

use super::*;
use crate::syscalls::*;

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
    todo!();
}
