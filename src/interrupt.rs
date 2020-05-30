//! Interrupts

pub use bare_metal::CriticalSection;

/// Disables all interrupts and return the previous settings
#[inline]
pub fn disable() -> u32 {
    unsafe { set_mask(0) }
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable() -> u32 {
    set_mask(!0)
}

/// Enables specific interrupts and returns the previous setting
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn set_mask(mut mask: u32) -> u32 {
    llvm_asm!("
        xsr $0, intenable
        rsync
        " :"=r"(mask) :"0"(mask):: "volatile");
    mask
}

/// Disables specific interrupts and returns the previous settings
#[inline]
pub fn disable_mask(mask: u32) -> u32 {
    let mut prev: u32 = 0;
    let _dummy: u32;
    unsafe {
        llvm_asm!("
        xsr.intenable $0  // get mask and temporarily disable interrupts 
        and $1,$1,$0
        rsync
        wsr.intenable $1
        rsync
    " : "+r"(prev),"=r"(_dummy) : "1"(!mask) ::"volatile");
    }
    prev
}

/// Enables specific interrupts and returns the previous setting
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable_mask(mask: u32) -> u32 {
    let mut prev: u32 = 0;
    let _dummy: u32;
    llvm_asm!("
        xsr.intenable $0 // get mask and temporarily disable interrupts
        or $1,$1,$0
        rsync
        wsr.intenable $1
        rsync
    " : "+r"(prev),"=r"(_dummy) : "1"(mask)::"volatile");
    prev
}

/// Get current interrupt mask
#[inline]
pub fn get_mask() -> u32 {
    let mask: u32;
    unsafe { llvm_asm!("rsr.intenable $0" : "=r"(mask) ) };
    mask
}

/// Get currently active interrupts
#[inline]
pub fn get() -> u32 {
    let mask: u32;
    unsafe {
        llvm_asm!("rsr.interrupt $0":"=r"(mask):::"volatile");
    }
    mask
}

/// Set interrupt
///
/// Only valid for software interrupts
#[inline]
pub unsafe fn set(mask: u32) {
    llvm_asm!("wsr.interrupt $0"::"r"(mask)::"volatile");
}

/// Clear interrupt
///
/// Only valid for software and edge-triggered interrupts
#[inline]
pub unsafe fn clear(mask: u32) {
    llvm_asm!("wsr.intclear $0"::"r"(mask)::"volatile");
}

/// Get current interrupt level
#[inline]
pub fn get_level() -> u32 {
    let ps: u32;
    unsafe { llvm_asm!("rsr.ps $0":"=r"(ps):::"volatile") };
    ps & 0xf
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    // disable interrupts and store old mask
    let old_mask = disable();

    let r = f(unsafe { &CriticalSection::new() });

    // enable previously disable interrupts
    unsafe {
        enable_mask(old_mask);
    }

    r
}
