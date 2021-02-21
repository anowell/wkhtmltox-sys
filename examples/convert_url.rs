#[macro_use]
extern crate lazy_static;
extern crate wkhtmltox_sys;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use wkhtmltox_sys::{pdf::*, *};

lazy_static! {
    static ref LIB: wkhtmltox = unsafe { wkhtmltox::new(get_library_name().as_str()).unwrap() };
}

unsafe extern "C" fn finished(_converter: *mut wkhtmltopdf_converter, val: c_int) {
    println!("Finished: {}", val);
}

unsafe extern "C" fn error_cb(_converter: *mut wkhtmltopdf_converter, ptr: *const c_char) {
    let msg = CStr::from_ptr(ptr).to_string_lossy();
    println!("Error: {}", msg);
}

unsafe extern "C" fn warning_cb(_converter: *mut wkhtmltopdf_converter, ptr: *const c_char) {
    let msg = CStr::from_ptr(ptr).to_string_lossy();
    println!("Warning: {}", msg);
}

unsafe extern "C" fn progress_changed(_converter: *mut wkhtmltopdf_converter, val: c_int) {
    println!("{:3}", val);
}

unsafe extern "C" fn phase_changed(converter: *mut wkhtmltopdf_converter) {
    let phase = LIB.wkhtmltopdf_current_phase(converter);
    let desc = LIB.wkhtmltopdf_phase_description(converter, phase);
    println!("Phase: {}", CStr::from_ptr(desc).to_string_lossy());
}

fn main() {
    unsafe {
        let version = CStr::from_ptr(LIB.wkhtmltopdf_version()).to_string_lossy();
        println!("Version: {}", version);

        // Init wkhtmltopdf in graphics-less mode
        if LIB.wkhtmltopdf_init(0) != 1 {
            return println!("Init failed");
        }

        let gs = LIB.wkhtmltopdf_create_global_settings();
        let os = LIB.wkhtmltopdf_create_object_settings();
        let converter = LIB.wkhtmltopdf_create_converter(gs);
        LIB.wkhtmltopdf_set_object_setting(
            os,
            CString::new("page").unwrap().as_c_str().as_ptr(),
            CString::new("https://wkhtmltopdf.org")
                .unwrap()
                .as_c_str()
                .as_ptr(),
        );
        LIB.wkhtmltopdf_add_object(converter, os, std::ptr::null());
        std::mem::drop(os);

        // Setup callbacks
        LIB.wkhtmltopdf_set_finished_callback(converter, Some(finished));
        LIB.wkhtmltopdf_set_progress_changed_callback(converter, Some(progress_changed));
        LIB.wkhtmltopdf_set_phase_changed_callback(converter, Some(phase_changed));
        LIB.wkhtmltopdf_set_error_callback(converter, Some(error_cb));
        LIB.wkhtmltopdf_set_warning_callback(converter, Some(warning_cb));

        // Perform the conversion
        if LIB.wkhtmltopdf_convert(converter) != 1 {
            println!("Conversion failed");
        }
        {
            let mut data = std::ptr::null();
            let bytes = LIB.wkhtmltopdf_get_output(converter, &mut data) as usize;
            println!("Received {} bytes", bytes);
            let pdf_buf = std::slice::from_raw_parts(data, bytes);
            std::fs::write("output.pdf", pdf_buf).unwrap();
        }

        LIB.wkhtmltopdf_destroy_converter(converter);
        LIB.wkhtmltopdf_deinit();
    }
}
