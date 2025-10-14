use common::{REP, ADDSS_XMM, ADDSS_OPCODE, SSE_PREFIX, XORPS_OPCODE, REG_XMM6_XMM6, NOP};
use common::{ DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH };
use windows::Win32::Foundation::HMODULE;

const OFFSET:   usize   = 0x583a5d;
const ORIGINAL: [u8; 4] = [REP, ADDSS_XMM, ADDSS_OPCODE, 0xF0];
const PATCHED:  [u8; 4] = [SSE_PREFIX, XORPS_OPCODE, REG_XMM6_XMM6, NOP];

#[allow(non_snake_case)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(_: HMODULE, reason: u32, _: *mut ()) -> bool {
    match reason {
        DLL_PROCESS_ATTACH => common::patch_bytes(OFFSET,   &ORIGINAL, &PATCHED).ok(),
        DLL_PROCESS_DETACH => common::restore_bytes(OFFSET, &ORIGINAL).ok(),
        _ => None,
    };

    true
}