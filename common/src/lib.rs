use std::ptr;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{
    VirtualProtect,
    VirtualAlloc,
    VirtualFree,
    PAGE_EXECUTE_READWRITE,
    PAGE_PROTECTION_FLAGS,
    MEM_COMMIT,
    MEM_RESERVE,
    MEM_RELEASE,
    VIRTUAL_ALLOCATION_TYPE,
};

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn get_base_address() -> Result<usize, String> {
    let base = GetModuleHandleA(None).map_err(|e| format!("Failed to get base module: {:?}", e))?;
    Ok(base.0 as usize)
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn patch_bytes(offset: usize, original: &[u8], patched: &[u8]) -> Result<(), String> {
    let base: usize   = get_base_address()?;
    let addr: *mut u8 = (base + offset) as *mut u8;

    let current = std::slice::from_raw_parts(addr, original.len());
    if current != original {
        return Err(
            format!(
                "Bytes don't match in offset 0x{:X}! Expected: {:02X?}, Founded: {:02X?}",
                offset,
                original,
                current
            )
        );
    }


    let mut old_protect = PAGE_PROTECTION_FLAGS(0);
    VirtualProtect(
        addr as *const _,
        patched.len(),
        PAGE_EXECUTE_READWRITE,
        &mut old_protect
    ).map_err(|e| format!("Failed to change protection: {:?}", e))?;

    ptr::copy_nonoverlapping(patched.as_ptr(), addr, patched.len());

    VirtualProtect(addr as *const _, patched.len(), old_protect, &mut old_protect).ok();

    Ok(())
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn restore_bytes(offset: usize, original: &[u8]) -> Result<(), String> {
    let base: usize   = get_base_address()?;
    let addr: *mut u8 = (base + offset) as *mut u8;

    let mut old_protect = PAGE_PROTECTION_FLAGS(0);
    VirtualProtect(addr as *const _, original.len(), PAGE_EXECUTE_READWRITE, &mut old_protect).ok();

    ptr::copy_nonoverlapping(original.as_ptr(), addr, original.len());

    VirtualProtect(addr as *const _, original.len(), old_protect, &mut old_protect).ok();

    Ok(())
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn allocate_memory(size: usize) -> Result<*mut u8, String> {
    let addr = VirtualAlloc(
        None,
        size,
        VIRTUAL_ALLOCATION_TYPE(MEM_COMMIT.0 | MEM_RESERVE.0),
        PAGE_EXECUTE_READWRITE
    );

    if addr.is_null() {
        Err("Failed to allocate memory".to_string())
    } else {
        Ok(addr as *mut u8)
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn free_memory(addr: *mut u8) {
    VirtualFree(addr as *mut _, 0, MEM_RELEASE).ok();
}
