//! Low-level bindings for wkhtmltopdf and wkhtmltoimage
//!
//! These bindings currently depend on libwkhtmltox 0.12

use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};

/// Convert HTML to PDF
///
/// A basic example of using wkhtmltopdf looks like this:
///
/// ```
/// use wkhtmltox_sys::{pdf::*, *};
/// use std::ffi::CString;
///
/// lazy_static! {
///     static ref LIB: wkhtmltox = unsafe { wkhtmltox::new(get_library_name().as_str()).unwrap() };
/// }
///
/// let html = CString::new(r##"<b>foo</b>bar"##).expect("null byte found");
/// unsafe {
///     // Init wkhtmltopdf in graphics-less mode
///     if LIB.wkhtmltopdf_init(0) != 1 {
///         panic!("Init failed");
///     }
///
///     let gs = LIB.wkhtmltopdf_create_global_settings();
///     let converter = LIB.wkhtmltopdf_create_converter(gs);
///
///     // Add one or more objects to the converter
///     let os = LIB.wkhtmltopdf_create_object_settings();
///     LIB.wkhtmltopdf_add_object(converter, os, html.as_ptr());
///
///     // Perform the conversion
///     if LIB.wkhtmltopdf_convert(converter) != 1 {
///         panic!("Conversion failed");
///     } {
///         // Read the data into `pdf_buf: Vec<u8>`
///         let mut data = std::ptr::null();
///         let bytes = LIB.wkhtmltopdf_get_output(converter, &mut data) as usize;
///         let pdf_buf = std::slice::from_raw_parts(data, bytes);
///         std::fs::write("output.pdf", pdf_buf).unwrap();
///     }
///
///     LIB.wkhtmltopdf_destroy_converter(converter);
///     LIB.wkhtmltopdf_deinit();
/// }
/// ```
///
/// Additional documentation:
///
/// - [Using PDF c-bindings](http://wkhtmltopdf.org/libwkhtmltox/)
/// - [PDF settings](http://wkhtmltopdf.org/libwkhtmltox/pagesettings.html) or [src](https://github.com/wkhtmltopdf/wkhtmltopdf/blob/44d13596af84e34d0ad5086dfc70a792c9bd623d/src/lib/pdfsettings.cc)
/// - [pdf.h bindings](http://wkhtmltopdf.org/libwkhtmltox/pdf_8h.html) or [src](https://github.com/wkhtmltopdf/wkhtmltopdf/blob/b8f9f5e354b22d459e1906cf18165cd21720cd83/src/lib/pdf_c_bindings.cc)
pub mod pdf;

/// Convert HTML to image
pub mod image;

/// Return the `wkhtmltox` library name
pub fn get_library_name() -> String {
    format!("{}wkhtmltox{}", DLL_PREFIX, DLL_SUFFIX)
}
