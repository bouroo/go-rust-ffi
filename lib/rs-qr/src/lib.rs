use base64::{engine::general_purpose, Engine as _};
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use std::{fs, ptr, str};

use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;

#[no_mangle]
pub extern "C" fn gen_qr_img(data: *const libc::c_char, file: *const libc::c_char) {
    let data = unsafe { CStr::from_ptr(data).to_bytes() };
    let file = unsafe { CStr::from_ptr(file).to_bytes() };

    let file_path = PathBuf::from(str::from_utf8(file).unwrap());
    let parent_dir = file_path.parent().unwrap_or(Path::new("./tmp"));

    fs::create_dir_all(parent_dir).unwrap();

    let qrcode = QRBuilder::new(str::from_utf8(data).unwrap())
        .build()
        .expect("Failed to build QR code");

    let _img = ImageBuilder::default()
        .shape(Shape::RoundedSquare)
        .background_color([255, 255, 255, 191])
        .fit_width(512)
        .to_file(&qrcode, file_path.to_str().unwrap())
        .expect("Failed to generate QR code");
}

#[no_mangle]
pub extern "C" fn gen_qr_base64(data: *const libc::c_char) -> *mut libc::c_char {
    let data_cstr = unsafe { CStr::from_ptr(data) };
    let data = data_cstr.to_str().expect("Invalid UTF-8 data");

    // Handle potential failure
    let qrcode = match QRBuilder::new(data).build() {
        Ok(qr) => qr,
        Err(e) => {
            eprintln!("Failed to build QR code: {}", e);
            return std::ptr::null_mut();
        }
    };

    let _img = ImageBuilder::default()
        .shape(Shape::RoundedSquare)
        .background_color([255, 255, 255, 191])
        .fit_width(512)
        .to_bytes(&qrcode)
        .expect("Failed to generate QR code");

    let img_str = format!(
        "data:image/png;base64,{}",
        general_purpose::STANDARD.encode(&_img)
    );
    let cstr_img = match CString::new(img_str) {
        Ok(cstr) => cstr,
        Err(e) => {
            eprintln!("Failed to create CString: {}", e);
            return ptr::null_mut();
        }
    };

    cstr_img.into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_gen_qr() {
        let input = CString::new("Hello, world!").unwrap().into_raw();
        let output_file = CString::new("tmp/qr.png").unwrap().into_raw();

        gen_qr_img(input, output_file);
        gen_qr_base64(input);
    }
}
