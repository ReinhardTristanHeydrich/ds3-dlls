use common::{MOV_EDX_EBX, REG_EBX, XOR_EDX_EDX, REG_EDX_EDX};
use common::{ DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH };
use windows::Win32::Foundation::HMODULE;

const OFFSET:   usize   = 0x58bfe4;
const ORIGINAL: [u8; 2] = [MOV_EDX_EBX, REG_EBX];
const PATCHED:  [u8; 2] = [XOR_EDX_EDX, REG_EDX_EDX];

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(_: HMODULE, reason: u32, _: *mut ()) -> bool {
    match reason {
        DLL_PROCESS_ATTACH => common::patch_bytes(OFFSET, &ORIGINAL, &PATCHED).ok(),
        DLL_PROCESS_DETACH => common::restore_bytes(OFFSET, &ORIGINAL).ok(),
        _ => None,
    };

    true
}