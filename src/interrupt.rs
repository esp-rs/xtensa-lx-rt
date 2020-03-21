//! Interrupts

pub use bare_metal::CriticalSection;

/// Disables all interrupts and return the previous settings
#[inline]
pub fn disable() -> u32 {
    unsafe { disable_mask(!0) }
}

/// Disables specific  interrupts and returns the previous settings
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn disable_mask(mask: u32) -> u32 {
    asm!("mov a2, $0" :: "r"(mask) :: "volatile"); /* enabled (1 << 6) */
    asm!("movi a3, 0" :::: "volatile");
    asm!("xsr.intenable a3" :::: "volatile"); /* Disable all interrupts */
    asm!("rsync");
    asm!("xor a2, a2, a2" :::: "volatile"); /* invert bit mask */
    asm!("and a2, a3, a2" :::: "volatile"); /* clear bits in mask */
    asm!("wsr.intenable a2" :::: "volatile"); /* Re-enable interrupts */
    asm!("rsync");

    let prev: u32;
    asm!("mov a2, a3" : "={a2}"(prev) ::: "volatile"); /* return prev mask */

    prev
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable() -> u32 {
    enable_mask(!0)
}

/// Enables specific interrupts and returns the previous setting
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable_mask(mask: u32) -> u32 {
    asm!("mov a2, $0" :: "r"(mask) :: "volatile"); /* enabled (1 << 6) */
    asm!("movi a3, 0" :::: "volatile");
    asm!("xsr.intenable a3" :::: "volatile"); /* Disable all interrupts */
    asm!("rsync");
    asm!("or a2, a3, a2" :::: "volatile"); /* set bits in mask */
    asm!("wsr.intenable a2" :::: "volatile"); /* Re-enable interrupts */
    asm!("rsync");

    let prev: u32;
    asm!("mov a2, a3" : "={a2}"(prev) ::: "volatile"); /* return prev mask */

    prev
}

/// Get current interrupt mask
#[inline]
pub fn get_mask() -> u32 {
    let x: u32;
    unsafe { asm!("rsr.intenable a2" : "={a2}"(x) ) };
    x
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
