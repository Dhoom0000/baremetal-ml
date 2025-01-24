pub unsafe fn read(addr:u64) -> u32 { unsafe {
    core::ptr::read_volatile(addr as *const u32)
}}

pub unsafe fn write(addr:u64,value:u32) { unsafe {
    core::ptr::write_volatile(addr as *mut u32,value);
}}