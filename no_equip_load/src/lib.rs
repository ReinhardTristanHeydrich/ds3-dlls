/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
 /*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ .  Compile-Time ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . */
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
/*✦══════════════════════✦ Imports ✦══════════════════════✦*/
use common::DLL_PROCESS_ATTACH;
use common::{REP, ADDSS_XMM, ADDSS_OPCODE, SSE_PREFIX, XORPS_OPCODE, REG_XMM6_XMM6, NOP};

/*✦───────── Thread ─────────✦*/
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::processthreadsapi::CreateThread;
use winapi::shared::minwindef::{LPVOID, DWORD};
use winapi::um::handleapi::CloseHandle;
use std::ptr;

/*✦══════════════════════✦ Consts ✦══════════════════════✦*/
const OFFSET:   usize   = 0x583a5d;
const ORIGINAL: [u8; 4] = [REP, ADDSS_XMM, ADDSS_OPCODE, 0xF0];
const PATCHED:  [u8; 4] = [SSE_PREFIX, XORPS_OPCODE, REG_XMM6_XMM6, NOP];

/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
 /*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ ✦ Code ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ . ✦*/
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "system" fn main(_: LPVOID) -> DWORD {
    /*✦═════════════════════════════════════════════✦══════════════════════════════════════════════✦*/
    common::patch_bytes(OFFSET,   &ORIGINAL, &PATCHED).ok();
    /*✦═════════════════════════════════════════════✦══════════════════════════════════════════════✦*/
    0
}

/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
 /*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺  Dll-Main . ⁺ 　. ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ . ✦*/
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(hmodule: usize, reason: u32, _: *mut ()) -> bool {
    if reason != DLL_PROCESS_ATTACH {return true}

    DisableThreadLibraryCalls(hmodule as *mut _);
    let thread = CreateThread(
        ptr::null_mut(),
        0,
        Some(main),
        ptr::null_mut(),
        0,
        ptr::null_mut(),
    );
    if !thread.is_null() {
        CloseHandle(thread);
    }

    true
}