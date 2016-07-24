//! Low-level bindings for wkhtmltopdf and wkhtmltoimage
//!
//! These bindings currently depend on libwkhtmltox 0.12.3

/// Convert HTML to PDF
///
/// A basic example of using wkhtmltopdf looks like this:
///
/// ```
/// use wkhtmltox_sys::pdf::*;
/// use std::ffi::CString;
///
/// let html = CString::new(r##"<b>foo</b>bar"##).expect("null byte found");
/// unsafe {
///     // Init wkhtmltopdf in graphics-less mode
///     if wkhtmltopdf_init(0) != 1 {
///         panic!("Init failed");
///     }
///
///     let gs = wkhtmltopdf_create_global_settings();
///     let converter = wkhtmltopdf_create_converter(gs);
///
///     // Add one or more objects to the converter
///     let os = wkhtmltopdf_create_object_settings();
///     wkhtmltopdf_add_object(converter, os, html.as_ptr());
///
///     // Perform the conversion
///     if wkhtmltopdf_convert(converter) != 1 {
///         panic!("Conversion failed");
///     } {
///         // Read the data into `pdf_buf: Vec<u8>`
///         let mut data = std::ptr::null();
///         let bytes = wkhtmltopdf_get_output(converter, &mut data) as usize;
///         let _pdf_buf = std::slice::from_raw_parts(data, bytes);
///     }
///
///     wkhtmltopdf_destroy_converter(converter);
///     wkhtmltopdf_deinit();
/// }
/// ```
///
/// Additional documentation:
///
/// - [Using PDF c-bindings](http://wkhtmltopdf.org/libwkhtmltox/)
/// - [PDF settings](http://wkhtmltopdf.org/libwkhtmltox/pagesettings.html) or [src](https://github.com/wkhtmltopdf/wkhtmltopdf/blob/4fa8338092ae77a4dd2e97c6e2e5b853e0c3005f/src/lib/pdfsettings.cc)
/// - [pdf.h bindings](http://wkhtmltopdf.org/libwkhtmltox/pdf_8h.html) or [src](https://github.com/wkhtmltopdf/wkhtmltopdf/blob/master/src/lib/pdf_c_bindings.cc)
pub mod pdf;

/// Convert HTML to image
pub mod image;