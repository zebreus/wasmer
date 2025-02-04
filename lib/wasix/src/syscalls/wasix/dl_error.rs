use std::{fs::OpenOptions, io::BufReader};

use super::*;
use crate::syscalls::*;

/// ### `dl_error()`
/// Get the most recent error from `dl_open`, `dl_close`, or `dl_load_symbol`
#[instrument(level = "trace", skip_all, fields(name = field::Empty), ret)]
pub fn dl_error<M: MemorySize>(
    mut ctx: FunctionEnvMut<'_, WasiEnv>,
    buffer: WasmPtr<u8, M>,
    buffer_len: M::Offset,
    error_length: WasmPtr<u32, M>,
) -> Result<Errno, WasiError> {
    let env = ctx.data();
    let (memory, mut state) = unsafe { env.get_memory_and_wasi_state(&ctx, 0) };

    if buffer_len.into() == 0 {
        return Ok(Errno::Fault);
    }
    // For now this always returns no error
    // TODO: Actually implement dl_error
    wasi_try_mem_ok!(buffer.write(&memory, 0));
    wasi_try_mem_ok!(error_length.write(&memory, 0));

    Ok(Errno::Success)
}
