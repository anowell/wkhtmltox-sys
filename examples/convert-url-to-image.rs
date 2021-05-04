extern crate wkhtmltox_sys;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use wkhtmltox_sys::image::*;

unsafe extern "C" fn finished(_converter: *mut wkhtmltoimage_converter, val: c_int) {
    println!("Finished: {}", val);
}

unsafe extern "C" fn error_cb(_converter: *mut wkhtmltoimage_converter, ptr: *const c_char) {
    let msg = CStr::from_ptr(ptr).to_string_lossy();
    println!("Error: {}", msg);
}

unsafe extern "C" fn warning_cb(_converter: *mut wkhtmltoimage_converter, ptr: *const c_char) {
    let msg = CStr::from_ptr(ptr).to_string_lossy();
    println!("Warning: {}", msg);
}

unsafe extern "C" fn progress_changed(_converter: *mut wkhtmltoimage_converter, val: c_int) {
    println!("{:3}", val);
}

unsafe extern "C" fn phase_changed(converter: *mut wkhtmltoimage_converter) {
    let phase = wkhtmltoimage_current_phase(converter);
    let desc = wkhtmltoimage_phase_description(converter, phase);
    println!("Phase: {}", CStr::from_ptr(desc).to_string_lossy());
}

fn main() {
    unsafe {
        let version = CStr::from_ptr(wkhtmltoimage_version()).to_string_lossy();
        println!("Version: {}", version);

        // Init wkhtmltoimage in graphics-less mode
        if wkhtmltoimage_init(0) != 1 {
            return println!("Init failed");
        }

        let gs = wkhtmltoimage_create_global_settings();
        wkhtmltoimage_set_global_setting(
            gs,
            CString::new("in").unwrap().as_ptr(),
            CString::new("https://rust-lang.org/en-US/")
                .unwrap()
                .as_ptr(),
        );
        wkhtmltoimage_set_global_setting(
            gs,
            CString::new("fmt").unwrap().as_ptr(),
            CString::new("png").unwrap().as_ptr(),
        );
        let converter = wkhtmltoimage_create_converter(gs, &0);

        // Setup callbacks
        wkhtmltoimage_set_finished_callback(converter, Some(finished));
        wkhtmltoimage_set_progress_changed_callback(converter, Some(progress_changed));
        wkhtmltoimage_set_phase_changed_callback(converter, Some(phase_changed));
        wkhtmltoimage_set_error_callback(converter, Some(error_cb));
        wkhtmltoimage_set_warning_callback(converter, Some(warning_cb));

        // Perform the conversion
        if wkhtmltoimage_convert(converter) != 1 {
            println!("Conversion failed");
        }
        {
            let mut data = std::ptr::null();
            let bytes = wkhtmltoimage_get_output(converter, &mut data) as usize;
            println!("Received {} bytes", bytes);
            let _image_buf = std::slice::from_raw_parts(data, bytes);
        }

        wkhtmltoimage_destroy_converter(converter);
        wkhtmltoimage_deinit();
    }
}
