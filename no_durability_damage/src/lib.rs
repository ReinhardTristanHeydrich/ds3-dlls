/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
 /*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ .  Compile-Time ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . */
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
/*✦══════════════════════✦ Imports ✦══════════════════════✦*/
use common::DLL_PROCESS_ATTACH;
use common::{MOV_EDX_EBX, REG_EBX, XOR_EDX_EDX, REG_EDX_EDX};

/*✦───────── Thread ─────────✦*/
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::processthreadsapi::CreateThread;
use winapi::shared::minwindef::{LPVOID, DWORD};
use winapi::um::handleapi::CloseHandle;
use std::ptr;

/*✦══════════════════════✦ Consts ✦══════════════════════✦*/
const OFFSET:   usize   = 0x58bfe4;
const ORIGINAL: [u8; 2] = [MOV_EDX_EBX, REG_EBX];
const PATCHED:  [u8; 2] = [XOR_EDX_EDX, REG_EDX_EDX];

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