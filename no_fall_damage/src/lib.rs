/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
 /*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ .  Compile-Time ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . */
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
/*✦══════════════════════✦ Imports ✦══════════════════════✦*/
use windows::{ Win32::Foundation::HMODULE };
use std::ptr;
use common::{ ADDSS_XMM, MOVAPS_OPCODE, MODRM_B1, JMP_REL32, NOP};
use common::{ DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH };
/*✦──────────────✦ Thread ✦──────────────✦*/
use std::thread;
use std::time::Duration;
use windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls as DisableThreadCalls;

/*✦══════════════════════✦ Consts ✦══════════════════════✦*/
/*✦──────────────✦ Patch-Values ✦──────────────✦*/
const OFFSET: usize = 0x9c83ca;
const ORIGINAL: [u8; 7] = [ADDSS_XMM, MOVAPS_OPCODE, MODRM_B1, 0x70, 0x01, 0x00, 0x00]; // movups xmm6,[rcx+00000170]

/*✦══════════════════════✦ Static ✦══════════════════════✦*/
static mut ALLOCATED_MEM: *mut u8 = ptr::null_mut();

/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
 /*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ ✦ Code ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ . ✦*/
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
/*✦──────────────✦ DLL-Main ✦──────────────✦*/

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(hmodule: HMODULE, reason: u32, _: *mut ()) -> bool {
    if reason == DLL_PROCESS_DETACH {
        remove_fall_damage_patch().ok();
        return true;
    }

    let _ = DisableThreadCalls(hmodule);


    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));

        if reason == DLL_PROCESS_ATTACH {
            if let Err(_) = apply_fall_damage_patch() {
                return false;
            }
        }

        true
    });

    true
}

/*✦──────────────✦ Patcher Function ✦──────────────✦*/

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn apply_fall_damage_patch() -> Result<(), String> {
    let base = common::get_base_address()?;
    let hook_addr = (base + OFFSET) as *mut u8;

    let current = std::slice::from_raw_parts(hook_addr, 7);
    if current != ORIGINAL {
        return Err(format!("Bytes Didn't match! Found: {:02X?}", current));
    }

    let mem = common::allocate_memory(0x1000)?;
    ALLOCATED_MEM = mem;

    let mut pos = 0usize;

    mem.add(pos).write(ADDSS_XMM); pos += 1;
    mem.add(pos).write(MOVAPS_OPCODE); pos += 1;
    mem.add(pos).write(0x35); pos += 1;

    let offset_to_data: i32 = 5;
    ptr::write_unaligned(mem.add(pos) as *mut i32, offset_to_data);
    pos += 4;

    mem.add(pos).write(JMP_REL32); pos += 1;

    let return_target: isize = hook_addr.add(7) as isize;
    let jmp_source:    isize = mem.add(pos + 4) as isize;
    let jmp_offset:    i32   = (return_target - jmp_source) as i32;
    ptr::write_unaligned(mem.add(pos) as *mut i32, jmp_offset);
    pos += 4;

    ptr::write_unaligned(mem.add(pos) as *mut f32, 0.0f32);     pos += 4;
    ptr::write_unaligned(mem.add(pos) as *mut f32, -2000.0f32); pos += 4;
    ptr::write_unaligned(mem.add(pos) as *mut f32, 0.0f32);     pos += 4;
    ptr::write_unaligned(mem.add(pos) as *mut f32, 1.0f32);

    let mut old_protect = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    windows::Win32::System::Memory
        ::VirtualProtect(
            hook_addr as *const _,
            7,
            windows::Win32::System::Memory::PAGE_EXECUTE_READWRITE,
            &mut old_protect
        )
        .ok();

    hook_addr.write(JMP_REL32);

    let target = mem as isize;
    let source = hook_addr.add(5) as isize;
    let hook_offset = (target - source) as i32;
    ptr::write_unaligned(hook_addr.add(1) as *mut i32, hook_offset);

    hook_addr.add(5).write(NOP);
    hook_addr.add(6).write(NOP);

    windows::Win32::System::Memory
        ::VirtualProtect(hook_addr as *const _, 7, old_protect, &mut old_protect)
        .ok();

    Ok(())
}

/*✦──────────────✦ Un-Patcher Function ✦──────────────✦*/
#[allow(unsafe_op_in_unsafe_fn)]
#[allow(unused)]
unsafe fn remove_fall_damage_patch() -> Result<(), String> {
    if !ALLOCATED_MEM.is_null() {
        common::free_memory(ALLOCATED_MEM);
        ALLOCATED_MEM = ptr::null_mut();
    }

    common::restore_bytes(OFFSET, &ORIGINAL)
}
