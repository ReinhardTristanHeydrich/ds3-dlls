use windows::{core::BOOL, Win32::Foundation::HMODULE};
use std::ptr;

const OFFSET:   usize   = 0x9C83CA;
const ORIGINAL: [u8; 7] = [0x0F, 0x28, 0xB1, 0x70, 0x01, 0x00, 0x00]; // movups xmm6,[rcx+00000170]

static mut ALLOCATED_MEM: *mut u8 = ptr::null_mut();

#[allow(unsafe_op_in_unsafe_fn)]
#[unsafe(export_name = "DllMain")]
pub unsafe extern "system" fn main(_: HMODULE, reason: u32, _: *mut ()) -> BOOL {
    match reason {
        1 => { // DLL_PROCESS_ATTACH
            if let Err(_) = apply_fall_damage_patch() {
                return false.into();
            }
        },
        0 => { // DLL_PROCESS_DETACH
            remove_fall_damage_patch().ok();
        },
        _ => {}
    }
    true.into()
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn apply_fall_damage_patch() -> Result<(), String> {
    let base = common::get_base_address()?;
    let hook_addr = (base + OFFSET) as *mut u8;
    
    let current = std::slice::from_raw_parts(hook_addr, 7);
    if current != ORIGINAL {
        return Err(format!("Bytes não correspondem! Encontrado: {:02X?}", current));
    }
    
    let mem = common::allocate_memory(0x1000)?;
    ALLOCATED_MEM = mem;
    
    let mut pos = 0usize;
    

    mem.add(pos).write(0x0F); pos += 1;  // Prefixo movups
    mem.add(pos).write(0x28); pos += 1;  // Opcode movups
    mem.add(pos).write(0x35); pos += 1;  // ModRM: xmm6, [rip+disp32]
    

    let offset_to_data: i32 = 5;
    ptr::write_unaligned(mem.add(pos) as *mut i32, offset_to_data);
    pos += 4;
    
    mem.add(pos).write(0xE9); pos += 1;
    

    let return_target = hook_addr.add(7) as isize;
    let jmp_source = mem.add(pos + 4) as isize;
    let jmp_offset = (return_target - jmp_source) as i32;
    ptr::write_unaligned(mem.add(pos) as *mut i32, jmp_offset);
    pos += 4;
    

    ptr::write_unaligned(mem.add(pos) as *mut f32, 0.0f32); pos += 4;
    ptr::write_unaligned(mem.add(pos) as *mut f32, -2000.0f32); pos += 4;
    ptr::write_unaligned(mem.add(pos) as *mut f32, 0.0f32); pos += 4;
    ptr::write_unaligned(mem.add(pos) as *mut f32, 1.0f32);
    
    let mut old_protect = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    windows::Win32::System::Memory::VirtualProtect(
        hook_addr as *const _,
        7,
        windows::Win32::System::Memory::PAGE_EXECUTE_READWRITE,
        &mut old_protect
    ).ok();
    
    hook_addr.write(0xE9); // Opcode jmp rel32
    

    let target = mem as isize;
    let source = hook_addr.add(5) as isize;
    let hook_offset = (target - source) as i32;
    ptr::write_unaligned(hook_addr.add(1) as *mut i32, hook_offset);
    
    hook_addr.add(5).write(0x90);
    hook_addr.add(6).write(0x90);

    windows::Win32::System::Memory::VirtualProtect(
        hook_addr as *const _,
        7,
        old_protect,
        &mut old_protect
    ).ok();
    
    Ok(())
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn remove_fall_damage_patch() -> Result<(), String> {
    if !ALLOCATED_MEM.is_null() {
        common::free_memory(ALLOCATED_MEM);
        ALLOCATED_MEM = ptr::null_mut();
    }
    
    common::restore_bytes(OFFSET, &ORIGINAL)
}