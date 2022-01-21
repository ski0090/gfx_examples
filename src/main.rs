extern crate gfx_backend_vulkan as back;
use std::ffi::c_void;

use gfx_hal::{adapter::Adapter, Instance};

fn main() {
    let instance = back::Instance::create("test", 1).expect("Failed to create an instance!");

    let gpu = selected_gpu(&instance);

    let direct_display = match std::env::var("DIRECT_DISPLAY") {
        Ok(_) => true,
        Err(_) => false,
    };

    let hwnd: *mut c_void = c_void::__variant1;

    let surface = unsafe {
        instance
            .create_surface(&hwnd)
            .expect("Failed to create a surface!")
    };
}

fn selected_gpu<I: gfx_hal::Instance<B>, B: gfx_hal::Backend>(instance: &I) -> Adapter<B> {
    let mut adapters = instance.enumerate_adapters();
    for adapter in &adapters {
        println!("{:?}", adapter.info);
    }
    adapters.remove(0)
}
