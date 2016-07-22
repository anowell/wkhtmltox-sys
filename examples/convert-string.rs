extern crate libwkhtmltox_sys as libwkhtmltox;

use libwkhtmltox::pdf::*;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_uchar};

// unsafe extern fn finished(_converter: *mut wkhtmltopdf_converter, val: c_int) {
//     println!("Finished: {}", val);
// }

// unsafe extern fn error_cb(_converter: *mut wkhtmltopdf_converter, ptr: *const c_char) {
//     let msg = CStr::from_ptr(ptr).to_string_lossy();
//     println!("Error: {}", msg);
// }

// unsafe extern fn warning_cb(_converter: *mut wkhtmltopdf_converter, ptr: *const c_char) {
//     let msg = CStr::from_ptr(ptr).to_string_lossy();
//     println!("Warning: {}", msg);
// }

// unsafe extern fn progress_changed(_converter: *mut wkhtmltopdf_converter, val: c_int) {
//     println!("{:3}", val);
// }

// unsafe extern fn phase_changed(converter: *mut wkhtmltopdf_converter) {
//     let phase = wkhtmltopdf_current_phase(converter);
//     let desc = wkhtmltopdf_phase_description(converter, phase);
// 	println!("Phase: {}", CStr::from_ptr(desc).to_string_lossy());
// }

fn main() {
    let html = CString::new(r##"<b>foo</b>bar"##).expect("null byte found");

    unsafe {
        // Init wkhtmltopdf in graphics-less mode
        if wkhtmltopdf_init(0) != 1 {
            return println!("Init failed");
        }

        let gs = wkhtmltopdf_create_global_settings();
        let os = wkhtmltopdf_create_object_settings();
        let converter = wkhtmltopdf_create_converter(gs);

        // Setup callbacks
        // wkhtmltopdf_set_finished_callback(converter, Some(finished));
        // wkhtmltopdf_set_progress_changed_callback(converter, Some(progress_changed));
        // wkhtmltopdf_set_phase_changed_callback(converter, Some(phase_changed));
        // wkhtmltopdf_set_error_callback(converter, Some(error_cb));
        // wkhtmltopdf_set_warning_callback(converter, Some(warning_cb));

        // Perform the conversion
        wkhtmltopdf_add_object(converter, os, html.as_ptr());
        if wkhtmltopdf_convert(converter) != 1 {
            println!("Conversion failed");
        } {
            let mut data = std::ptr::null();
            println!("Calling wkhtmltopdf_get_output");
            let bytes = wkhtmltopdf_get_output(converter, &mut data) as usize;
            println!("Received {} bytes", bytes);
            let buf_slice = std::slice::from_raw_parts(data, bytes);
        }

        wkhtmltopdf_destroy_converter(converter);
        wkhtmltopdf_deinit();
    }
}

