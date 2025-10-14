use windows::{core::BOOL, Win32::Foundation::HMODULE};

const OFFSET:   usize   = 0x4948ae;
const ORIGINAL: [u8; 5] = [0xE8, 0x2D, 0xB9, 0xFB, 0xFF]; // call DarkSoulsIII.exe+4801e0
const PATCHED:  [u8; 5] = [0x90, 0x90, 0x90, 0x90, 0x90]; // nop nop nop nop nop

#[allow(unsafe_op_in_unsafe_fn)]
#[unsafe(export_name = "DllMain")]
pub unsafe extern "system" fn main(_: HMODULE, reason: u32, _: *mut ()) -> BOOL {
    match reason {
        1 => common::patch_bytes(OFFSET, &ORIGINAL, &PATCHED).ok(),
        0 => common::restore_bytes(OFFSET, &ORIGINAL).ok(),
        _ => None,
    };
    true.into()
}