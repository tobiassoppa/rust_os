//! The `kernel` binary.
//!
//! # TL;DR - Overview of important Kernel entities
//!
//! - [`bsp::console::console()`] - Returns a reference to the kernel's [console interface].
//! - [`bsp::driver::driver_manager()`] - Returns a reference to the kernel's [driver interface].
//!
//! [console interface]: ../libkernel/console/interface/index.html
//! [driver interface]: ../libkernel/driver/interface/trait.DriverManager.html
//!
//! # Code organization and architecture
//!
//! The code is divided into different *modules*, each representing a typical **subsystem** of the
//! <ommited for now>

#![allow(incomplete_features)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_generics)]
#![feature(const_panic)]
#![feature(format_args_nl)]
#![feature(naked_functions)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main] // disable all Rust-level entry points
#![no_std] // don't link Rust standard lib

mod bsp;
mod console;
mod cpu;
mod driver;
mod exception;
mod memory;
mod panic_wait;
mod print;
mod runtime_init;
mod synchronization;
mod time;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order:
///     - Virtual memory must be activated before the device drivers.
///         - Without it, any tomic operations, e.g. the yet-to-be-introduced spinlocks
///           in the device drivers (which currently employ NullLocks instead of spinlocks),
///           will fail to work on the RPi SoCs.
unsafe fn kernel_init() -> ! {
    use driver::interface::DriverManager;
    use memory::mmu::interface::MMU;

    if let Err(string) = memory::mmu::mmu().init() {
        panic!("MMU: {}", string);
    }

    for i in bsp::driver::driver_manager().all_device_drivers().iter() {
        if let Err(x) = i.init() {
            panic!("Error loading driver: {}: {}", i.compatible(), x);
        }
    }
    bsp::driver::driver_manager().post_device_driver_init();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    use console::interface::All;
    use core::time::Duration;
    use driver::interface::DriverManager;
    use time::interface::TimeManager;

    info!("Booting on: {}", bsp::board_name());

    info!("MMU online. Special regions:");
    bsp::memory::mmu::virt_mem_layout().print_layout();

    let (_, privilege_level) = exception::current_privilege_level();
    info!("Current privilgege level: {}", privilege_level);

    info!("Exception handling state:");
    exception::asynchronous::print_state();

    info!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    for (i, driver) in bsp::driver::driver_manager()
        .all_device_drivers()
        .iter()
        .enumerate()
    {
        info!("     {}.{}", i + 1, driver.compatible());
    }

    // Test a failing timer case.
    info!("Timer test, spinning for 1 second");
    time::time_manager().spin_for(Duration::from_secs(1));

    let remapped_uart = unsafe { bsp::device_driver::PL011Uart::new(0x1FFF_1000) };
    writeln!(
        remapped_uart,
        "[      !!!     ] Writing through the remapped UART at 0x1FFF_1000"
    )
    .unwrap();

    info!("Echoing input now");
    loop {
        let c = bsp::console::console().read_char();
        bsp::console::console().write_char(c);
    }
}
