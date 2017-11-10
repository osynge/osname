extern crate libc;


mod importer;


use std::ffi::CStr as CStr;
use std::boxed::Box;
use std::ffi::CString as CString;
use libc::c_char as c_char;

#[no_mangle]
pub extern fn how_many_characters(s: *const c_char) -> bool {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    true
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
pub extern fn osname_init_crypto() {
    importer::crypto_init();
}

pub extern fn wrapped_BIO_new_file(name: &'static str, mode: &'static str) {
    let lname = CString::new(name).unwrap();
    let lmode = CString::new(mode).unwrap();
    let plname = lname.as_ptr();
    let plmode = lmode.as_ptr();
    unsafe {
        let fp = importer::BIO_new_file(plname, plmode);
//        let x509 = importer::PEM_read_bio_X509(foo);
    };
    
}







#[repr(C)]
pub struct Info {
    confidence: f64
}


#[no_mangle]
pub extern fn whatlang_get_info() -> Box<Info> {
    let info = Info { confidence: 0.9 };
    Box::new(info)
}
