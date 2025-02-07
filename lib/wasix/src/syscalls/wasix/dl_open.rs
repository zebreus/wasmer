use super::*;
use crate::syscalls::*;
use std::{fs::OpenOptions, io::BufReader};
use wasmer::{imports, TableType, Type};

/// ### `dl_open()`
/// Open a dynamic library
#[instrument(level = "trace", skip_all, fields(name = field::Empty), ret)]
pub fn dl_open<M: MemorySize>(
    mut ctx: FunctionEnvMut<'_, WasiEnv>,
    filename: WasmPtr<u8, M>,
    filename_len: M::Offset,
    handle: WasmPtr<u32, M>,
) -> Result<Errno, WasiError> {
    // let env = ctx.data();
    // let (memory, mut state) = unsafe { env.get_memory_and_wasi_state(&ctx, 0) };
    // wasi_try_mem_ok!(handle.write(&memory, 5));
    // return Ok(Errno::Success);

    let env = ctx.data();
    let (memory, mut state) = unsafe { env.get_memory_and_wasi_state(&ctx, 0) };
    let filename = unsafe { get_input_str_ok!(&memory, filename, filename_len) };

    Span::current().record("filename", filename.as_str());

    let opened_handle = wasi_try_ok!(dl_open_internal(&mut ctx, &filename));

    let env = ctx.data();
    let (memory, mut state) = unsafe { env.get_memory_and_wasi_state(&ctx, 0) };

    wasi_try_mem_ok!(handle.write(&memory, opened_handle));

    Ok(Errno::Success)
}

pub fn dl_open_internal(
    ctx: &mut FunctionEnvMut<'_, WasiEnv>,
    filename: &str,
) -> Result<u32, Errno> {
    // Check if the directory exists
    let file = match OpenOptions::new().read(true).open(Path::new(filename)) {
        Ok(file) => file,
        Err(e) => {
            // TODO: Handle properly, currently the function just returns 0
            println!("error: {}", e);
            return Err(Errno::Noent);
        }
    };
    let opened_file = file.bytes().collect::<Result<Vec<u8>, _>>().unwrap();
    let original_instance = WasiEnv::experimental_dlopen(ctx, &opened_file).unwrap();

    Ok(5)
}
