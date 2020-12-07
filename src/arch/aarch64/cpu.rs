// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>

//! Architectural processor code.

use crate::{bsp, cpu};
use cortex_a::{asm, regs::*};

//--------------------------------------------------------------------------------------------------
// Boot Code
//--------------------------------------------------------------------------------------------------

/// The entry of the `kernel` binary.
///
/// The function must be named `_start`, because the linker is looking for this exact name.
///
/// # Safety
///
/// - Linker script must ensure to place this function at `0x8_0000`.
#[naked]
#[no_mangle]
pub unsafe fn _start() -> ! {
    use crate::runtime_init;

    // Expect the boot core to start in EL2.
    if bsp::cpu::BOOT_CORE_ID == cpu::smp::core_id() {
        SP.set(bsp::memory::boot_core_stack_end() as u64);
        runtime_init::runtime_init()
    } else {
        // If not core0, infinitely wait for events.
        wait_forever()
    }
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

pub use asm::nop;

/// Pause execution on the core.
#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}
