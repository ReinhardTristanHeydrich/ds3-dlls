use windows::{core::BOOL, Win32::Foundation::HMODULE};

const OFFSET:   usize   = 0x58bfe4;
const ORIGINAL: [u8; 2] = [0x8B, 0xD3]; // mov edx,ebx
const PATCHED:  [u8; 2] = [0x31, 0xD2]; // xor edx,edx


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