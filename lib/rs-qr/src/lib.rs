use base64::{engine::general_purpose, Engine as _};
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use std::{fs, ptr, str};

use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;

// Generate a QR code image and save it to a file
#[no_mangle]
pub extern "C" fn gen_qr_img(data: *const libc::c_char, file: *const libc::c_char) {
    // Convert the input C strings to Rust byte slices
    let data = unsafe { CStr::from_ptr(data).to_bytes() };
    let file = unsafe { CStr::from_ptr(file).to_bytes() };

    // Convert the file byte slice to a PathBuf
    let file_path = PathBuf::from(str::from_utf8(file).unwrap());

    // Get the parent directory of the file path, or use "./tmp" if it doesn't exist
    let parent_dir = file_path.parent().unwrap_or(Path::new("./tmp"));

    // Create the parent directory if it doesn't exist
    fs::create_dir_all(parent_dir).unwrap();

    // Build the QR code using the input data
    let qrcode = QRBuilder::new(str::from_utf8(data).unwrap())
        .build()
        .expect("Failed to build QR code");

    // Create an ImageBuilder with default settings
    let _img = ImageBuilder::default()
        .shape(Shape::RoundedSquare)
        .background_color([255, 255, 255, 191])
        .fit_width(512)
        .to_file(&qrcode, file_path.to_str().unwrap())
        .expect("Failed to generate QR code");
}

// Generate a base64-encoded representation of a QR code image
#[no_mangle]
pub extern "C" fn gen_qr_base64(data: *const libc::c_char) -> *mut libc::c_char {
    // Convert the input C string to a Rust CStr
    let data_cstr = unsafe { CStr::from_ptr(data) };

    // Convert the C string to a UTF-8 string slice
    let data = data_cstr.to_str().expect("Invalid UTF-8 data");

    // Build the QR code using the input data and handle potential failure
    let qrcode = match QRBuilder::new(data).build() {
        Ok(qr) => qr,
        Err(e) => {
            eprintln!("Failed to build QR code: {}", e);
            return ptr::null_mut();
        }
    };

    // Create an ImageBuilder with default settings
    let _img = ImageBuilder::default()
        .shape(Shape::RoundedSquare)
        .background_color([255, 255, 255, 191])
        .fit_width(512)
        .to_bytes(&qrcode)
        .expect("Failed to generate QR code");

    // Encode the image bytes using base64 encoding
    let img_str = format!(
        "data:image/png;base64,{}",
        general_purpose::STANDARD.encode(&_img)
    );

    // Create a CString from the encoded string and handle potential failure
    let cstr_img = match CString::new(img_str) {
        Ok(cstr) => cstr,
        Err(e) => {
            eprintln!("Failed to create CString: {}", e);
            return ptr::null_mut();
        }
    };

    // Convert the CString into a raw pointer
    cstr_img.into_raw()
}

// Unit tests
#[cfg(test)]
mod tests {
    // Import the contents of the parent module
    use super::*;
    // Import the CString type from the std::ffi module
    use std::ffi::CString;

    // Define a unit test function named "test_gen_qr"
    #[test]
    fn test_gen_qr() {
        // Create a new CString containing the string "Hello, world!"
        let input = CString::new("Hello, world!").unwrap().into_raw();
        // Create a new CString containing the string "tmp/qr.png"
        let output_file = CString::new("tmp/qr.png").unwrap().into_raw();

        // Call the gen_qr_img function with the input and output_file arguments
        gen_qr_img(input, output_file);
        // Call the gen_qr_base64 function with the input argument
        gen_qr_base64(input);
    }
}
