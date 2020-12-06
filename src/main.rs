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

#![feature(asm)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(core_intrinsics)]
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
mod memory;
mod panic_wait;
mod print;
mod relocate;
mod runtime_init;
mod synchronization;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order.
unsafe fn kernel_init() -> ! {
    use driver::interface::DriverManager;

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
    use bsp::console::console;
    use console::interface::All;

    println!(" __  __ _      _ _                 _ ");
    println!("|  \\/  (_)_ _ (_) |   ___  __ _ __| |");
    println!("| |\\/| | | ' \\| | |__/ _ \\/ _` / _` |");
    println!("|_|  |_|_|_||_|_|____\\___/\\__,_\\__,_|");
    println!();
    println!("{:^37}", bsp::board_name());
    println!();
    println!("[ML] Requesting binary");
    console().flush();

    // Clear the RX FIFOs, if any, of spurious received characters before starting with the loader
    // protocol.
    console().clear();

    // Notify `Minipush` to send the binary.
    for _ in 0..3 {
        console().write_char(3 as char);
    }

    // Read the binary's size.
    let mut size: u32 = u32::from(console().read_char() as u8);
    size |= u32::from(console().read_char() as u8) << 8;
    size |= u32::from(console().read_char() as u8) << 16;
    size |= u32::from(console().read_char() as u8) << 24;

    // Trust it's not too big.
    console().write_char('O');
    console().write_char('K');

    let kernel_addr: *mut u8 = bsp::memory::board_default_load_addr() as *mut u8;
    unsafe {
        // Read the kernel byte by byte
        for i in 0..size {
            core::ptr::write_volatile(kernel_addr.offset(i as isize), console().read_char() as u8)
        }
    }

    println!("[ML] Loaded! Executing the payload now\n");
    console().flush();

    // Use black magic to get a function pointer.
    let kernel: fn() -> ! = unsafe { core::mem::transmute(kernel_addr) };

    // jump to loaded kernel!
    kernel()
}
