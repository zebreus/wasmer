use std::{fs::OpenOptions, io::BufReader};

use super::*;
use crate::syscalls::*;

/// ### `dl_close()`
/// Closes a previously opened dynamic library
#[instrument(level = "trace", skip_all, fields(name = field::Empty), ret)]
pub fn dl_close<M: MemorySize>(
    mut ctx: FunctionEnvMut<'_, WasiEnv>,
    handle: WasmPtr<u32, M>,
) -> Result<Errno, WasiError> {
    todo!();
}
