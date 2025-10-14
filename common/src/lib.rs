use std::ptr;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use region::{ Protection, Allocation };

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn get_base_address() -> Result<usize, String> {
    let base = GetModuleHandleA(None).map_err(|e| format!("Failed to get base module: {:?}", e))?;
    Ok(base.0 as usize)
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn patch_bytes(offset: usize, original: &[u8], patched: &[u8]) -> Result<(), String> {
    let base: usize   = get_base_address()?;
    let addr: *mut u8 = (base + offset) as *mut u8;

    //Byte verification
    let current = std::slice::from_raw_parts(addr, original.len());
    if  current != original {
        return Err(
            format!(
                "Bytes didn't match in offset 0x{:X}! Expected: {:02X?}, Founded: {:02X?}",
                offset,
                original,
                current
            )
        );
    }

    region::protect(addr, patched.len(), Protection::READ_WRITE_EXECUTE)
    .map_err(|e| format!("Failed to change protection: {:?}", e))?;

    ptr::copy_nonoverlapping(patched.as_ptr(), addr, patched.len());

    Ok(())
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn restore_bytes(offset: usize, original: &[u8]) -> Result<(), String> {
    let base = get_base_address()?;
    let addr = (base + offset) as *mut u8;

    region::protect(addr, original.len(), Protection::READ_WRITE_EXECUTE)
    .map_err(|e| format!("Falha ao mudar proteção: {:?}", e))?;

    ptr::copy_nonoverlapping(original.as_ptr(), addr, original.len());

    Ok(())
}

pub unsafe fn allocate_memory(size: usize) -> Result<Allocation, String> {
    region::alloc(size, Protection::READ_WRITE_EXECUTE)
    .map_err(|e| format!("Failed to allocate memory: {:?}", e))
}

#[inline(always)]
#[deprecated(note = "Use `drop(alloc)` instead")]
pub unsafe fn free_memory(alloc: Allocation) {
    drop(alloc);
}
