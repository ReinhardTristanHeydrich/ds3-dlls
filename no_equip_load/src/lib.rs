use windows::{core::BOOL, Win32::Foundation::HMODULE};

const OFFSET:   usize   = 0x583a5d;
const ORIGINAL: [u8; 4] = [0xF3, 0x0F, 0x58, 0xF0];
const PATCHED:  [u8; 4] = [0x0F, 0x57, 0xF6, 0x90];

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