# BSP Code

BSP stands for Board Support Package. BSP code is organized under src/bsp.rs and contains target board specific definitions and functions. These are things such as the board's memory map or instances of drivers for devices that are featured on the respective board.

Just like processor architecture code, the BSP code's module structure tries to mirror the kernel's subsystem modules, but there is no transparent re-exporting this time. That means whatever is provided must be called starting from the bsp namespace, e.g. bsp::driver::driver_manager().

[Source](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/00_before_we_start#bsp-code)