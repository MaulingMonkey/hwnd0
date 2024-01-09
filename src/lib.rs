// crate
#![no_std]
#![doc = include_str!("../Readme.md")]
#![debugger_visualizer(natvis_file = "../hwnd.natvis")]

// warnings
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_patterns)] // probably improperly `match { ... }`ed constants



#[cfg(windows)] #[path = "interop/_interop.rs"] mod interop;
#[cfg(windows)] pub use values::*;
#[cfg(windows)] mod values {
    mod hwnd;           pub use hwnd::*;
    mod non_null_hwnd;  pub use non_null_hwnd::*;
}
