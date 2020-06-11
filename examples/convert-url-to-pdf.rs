extern crate wkhtmltox_sys;

use wkhtmltox_sys::pdf::*;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};

unsafe extern fn finished(_converter: *mut wkhtmltopdf_converter, val: c_int) {
    println!("Finished: {}", val);
}

unsafe extern fn error_cb(_converter: *mut wkhtmltopdf_converter, ptr: *const c_char) {
    let msg = CStr::from_ptr(ptr).to_string_lossy();
    println!("Error: {}", msg);
}

unsafe extern fn warning_cb(_converter: *mut wkhtmltopdf_converter, ptr: *const c_char) {
    let msg = CStr::from_ptr(ptr).to_string_lossy();
    println!("Warning: {}", msg);
}

unsafe extern fn progress_changed(_converter: *mut wkhtmltopdf_converter, val: c_int) {
    println!("{:3}", val);
}

unsafe extern fn phase_changed(converter: *mut wkhtmltopdf_converter) {
    let phase = wkhtmltopdf_current_phase(converter);
    let desc = wkhtmltopdf_phase_description(converter, phase);
	println!("Phase: {}", CStr::from_ptr(desc).to_string_lossy());
}

fn main() {


    unsafe {
        let version = CStr::from_ptr(wkhtmltopdf_version()).to_string_lossy();
        println!("Version: {}", version);

        // Init wkhtmltopdf in graphics-less mode
        if wkhtmltopdf_init(0) != 1 {
            return println!("Init failed");
        }

        let gs = wkhtmltopdf_create_global_settings();
        let os = wkhtmltopdf_create_object_settings();
        let converter = wkhtmltopdf_create_converter(gs);
        wkhtmltopdf_set_object_setting(os, CString::new("page").unwrap().as_ptr(), CString::new("https://rust-lang.org/en-US/").unwrap().as_ptr());
        wkhtmltopdf_add_object(converter, os, std::ptr::null());
        std::mem::drop(os);

        // Setup callbacks
        wkhtmltopdf_set_finished_callback(converter, Some(finished));
        wkhtmltopdf_set_progress_changed_callback(converter, Some(progress_changed));
        wkhtmltopdf_set_phase_changed_callback(converter, Some(phase_changed));
        wkhtmltopdf_set_error_callback(converter, Some(error_cb));
        wkhtmltopdf_set_warning_callback(converter, Some(warning_cb));


        // Perform the conversion
        if wkhtmltopdf_convert(converter) != 1 {
            println!("Conversion failed");
        } {
            let mut data = std::ptr::null();
            let bytes = wkhtmltopdf_get_output(converter, &mut data) as usize;
            println!("Received {} bytes", bytes);
            let _pdf_buf = std::slice::from_raw_parts(data, bytes);
        }

        wkhtmltopdf_destroy_converter(converter);
        wkhtmltopdf_deinit();
    }
}

