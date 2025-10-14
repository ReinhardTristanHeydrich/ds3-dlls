/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
/*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ .  Compile-Time ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . */
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
/*✦══════════════════════✦ Imports ✦══════════════════════✦*/
use windows::Win32::Foundation::HMODULE;
use common::{ NOP, CALL_REL32 };
use common::{ DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH };

/*✦══════════════════════✦ Consts ✦══════════════════════✦*/
const OFFSET:       usize   = 0x4948ae;
const ORIGINAL:     [u8; 5] = [CALL_REL32, 0x2d, 0xb9, 0xfb, 0xff]; // call DarkSoulsIII.exe+4801e0

const OFFSET_ALT:   usize   = 0x486531;
const ORIGINAL_ALT: [u8; 5] = [CALL_REL32, 0x1a, 0xbc, 0x04, 0x00]; // call darksoulsiii.exe+4d2150

const PATCHED: [u8; 5] = [NOP, NOP, NOP, NOP, NOP];

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(_: HMODULE, reason: u32, _: *mut ()) -> bool {
    match reason {
        DLL_PROCESS_ATTACH => {
            common::patch_bytes(OFFSET,     &ORIGINAL,     &PATCHED).ok();
            common::patch_bytes(OFFSET_ALT, &ORIGINAL_ALT, &PATCHED).ok();
        }
        DLL_PROCESS_DETACH => {
            common::restore_bytes(OFFSET,     &ORIGINAL).ok();
            common::restore_bytes(OFFSET_ALT, &ORIGINAL_ALT).ok();
        }
        _ => {},
    };

    true
}
