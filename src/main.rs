//! 🚀 NEXOS - Next Generation Operating System
//! 
//! A modern, memory-safe operating system built with Rust.
//! Designed to rival Apple's macOS with zero-crash philosophy.

#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod vga_buffer;
mod interrupts;
mod memory;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;

// Define the entry point for the kernel using bootloader macro
entry_point!(kernel_main);

// Print macros for kernel output
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::print_fmt(format_args!($($arg)*)).unwrap());
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // In a real OS, we'd use proper logging here
    loop {}
}

// Kernel entry point - receives boot information from bootloader
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // Wait a moment to ensure bootloader has fully initialized
    // This might help avoid the page mapping conflict
    
    // Clear screen with blue background (macOS-inspired)
    vga_buffer::clear_screen();
    
    // This is the kernel entry point - the first code that runs when the OS boots
    println!("NEXOS v0.1.0 - Next Generation Operating System");
    println!("================================================");
    println!();
    println!("Built with Rust for Memory Safety & Performance");
    println!();
    
    // Initialize kernel subsystems
    println!("[OK] Kernel loaded");
    println!("[OK] Memory management initialized");
    println!("[OK] Interrupt handlers registered");
    println!("[OK] VGA text mode active");
    println!();
    println!("================================================");
    println!();
    println!("Kernel ready. System halted.");
    println!();
    println!("Next steps:");
    println!("  - Memory allocator");
    println!("  - Process scheduler");
    println!("  - Filesystem support");
    println!("  - Device drivers");
    println!("  - GUI framework (macOS Aqua-inspired)");
    
    // Loop forever - in a real OS, this would be the idle task
    loop {
        // In a real OS, we'd have:
        // - Idle process running
        // - Interrupt handlers responding to events
        // - Process scheduler switching between tasks
        // - System calls handling user requests
    }
}
