/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
 /*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ .  Compile-Time ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . */
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
/*✦══════════════════════✦ Imports ✦══════════════════════✦*/
use common::DLL_PROCESS_ATTACH;
use common::{ NOP, CALL_REL32 };

/*✦───────── Thread ─────────✦*/
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::processthreadsapi::CreateThread;
use winapi::shared::minwindef::{LPVOID, DWORD};
use winapi::um::handleapi::CloseHandle;
use std::ptr;

/*✦══════════════════════✦ Consts ✦══════════════════════✦*/
const OFFSET:       usize   = 0x4948ae;
const OFFSET_ALT:   usize   = 0x486531;
const ORIGINAL:     [u8; 5] = [CALL_REL32, 0x2d, 0xb9, 0xfb, 0xff]; // call DarkSoulsIII.exe+4801e0
const ORIGINAL_ALT: [u8; 5] = [CALL_REL32, 0x1a, 0xbc, 0x04, 0x00]; // call darksoulsiii.exe+4d2150
const PATCHED:      [u8; 5] = [NOP, NOP, NOP, NOP, NOP];

/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
 /*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ ✦ Code ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ . ✦*/
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "system" fn main(_: LPVOID) -> DWORD {
    /*✦═════════════════════════════════════════════✦══════════════════════════════════════════════✦*/
    common::patch_bytes(OFFSET, &ORIGINAL, &PATCHED).ok();
    common::patch_bytes(OFFSET_ALT, &ORIGINAL_ALT, &PATCHED).ok();
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
