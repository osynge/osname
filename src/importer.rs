

#[allow(non_snake_case)]
pub enum BIO {}
#[allow(non_snake_case)]
pub enum X509 {}
#[allow(non_snake_case)]
pub enum X509_STORE {}
#[allow(non_snake_case)]
pub enum PKCS7 {}


use std::ptr;
use libc::c_char as c_char;
use std::ffi::CStr as CStr;
use std::os::raw::c_uint as c_uint;

pub const OPENSSL_INIT_NO_LOAD_CRYPTO_STRINGS: u64 = 1;
pub const OPENSSL_INIT_LOAD_CRYPTO_STRINGS: u64 = 2;
pub const OPENSSL_INIT_ADD_ALL_CIPHERS: u64 = 4;
pub const OPENSSL_INIT_ADD_ALL_DIGESTS: u64 = 8;
pub const OPENSSL_INIT_NO_ADD_ALL_CIPHERS: u64 = 16;
pub const OPENSSL_INIT_NO_ADD_ALL_DIGESTS: u64 = 32;
pub const OPENSSL_INIT_LOAD_CONFIG: u64 = 64;
pub const OPENSSL_INIT_NO_LOAD_CONFIG: u64 = 128;
pub const OPENSSL_INIT_ASYNC: u64 = 256;
pub const OPENSSL_INIT_ENGINE_RDRAND: u64 = 512;
pub const OPENSSL_INIT_ENGINE_DYNAMIC: u64 = 1024;
pub const OPENSSL_INIT_ENGINE_OPENSSL: u64 = 2048;
pub const OPENSSL_INIT_ENGINE_CRYPTODEV: u64 = 4096;
pub const OPENSSL_INIT_ENGINE_CAPI: u64 = 8192;
pub const OPENSSL_INIT_ENGINE_PADLOCK: u64 = 16384;
pub const OPENSSL_INIT_ENGINE_AFALG: u64 = 32768;
pub const OPENSSL_INIT_ENGINE_ALL_BUILTIN: u64 = 30208;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct conf_st {
    _unused: [u8; 0],
}

pub type CONF = conf_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ossl_init_settings_st {
    _unused: [u8; 0],
}
#[allow(non_snake_case)]
pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;



#[link(name = "ssl")]
#[link(name="crypto")]
extern {
    // redeclare functions from libcpuid.h here
    #[allow(non_snake_case)]
    pub fn OPENSSL_init_crypto(opts: u64, settings: *const OPENSSL_INIT_SETTINGS)
     -> ::std::os::raw::c_int;
    #[allow(non_snake_case)]
    pub fn BIO_new_file(name: *const c_char, mode: *const c_char) -> *mut BIO;
    #[allow(non_snake_case)]
    pub fn PEM_read_bio_X509(src_pem: *mut BIO, unknown_arg2: *const c_char, unknown_arg3: u32, unknown_arg3: *const c_char) -> *mut X509;
    #[allow(non_snake_case)]
    pub fn X509_STORE_new() -> * mut X509_STORE;
    #[allow(non_snake_case)]
    pub fn X509_STORE_add_cert(x509store: *mut X509_STORE, x509: *mut X509) -> u32;
    #[allow(non_snake_case)]
    pub fn SMIME_read_PKCS7(src: *mut BIO, bcont: *mut BIO) -> *mut PKCS7;
    #[allow(non_snake_case)]
    pub fn PKCS7_verify(p7: *mut PKCS7, unknown_arg1: *const c_char, store: *mut X509_STORE, content: *mut BIO, output: *mut BIO, unknown_arg2: u32);
    #[allow(non_snake_case)]
    pub fn PKCS7_free(pkcs7: *mut PKCS7);
    #[allow(non_snake_case)]
    pub fn X509_free(x509: *mut X509);
    #[allow(non_snake_case)]
    pub fn BIO_free(bio: *mut BIO);
}


pub fn crypto_init() {

    let ssl_init_modules : u64 = OPENSSL_INIT_ADD_ALL_CIPHERS + OPENSSL_INIT_ADD_ALL_DIGESTS + OPENSSL_INIT_LOAD_CONFIG + OPENSSL_INIT_LOAD_CRYPTO_STRINGS;

    unsafe {
        let rc = OPENSSL_init_crypto(ssl_init_modules, ptr::null());
   }
}



#[cfg(test)]

mod tests {
    use importer;
    use std::ffi::CString as CString;
    use std::ptr;
    #[test]
    fn X509_STORE_new()  {

        unsafe {
            importer::X509_STORE_new();
        };
    }
    #[test]
    fn OPENSSL_init_crypto()  {
		importer::crypto_init();
    }
    #[test]
    fn bio_new_file()  {
        importer::crypto_init();
        let name = "/tmp/foo";
        let mode = "r";

        let lname = CString::new(name).unwrap();
        let lmode = CString::new(mode).unwrap();
        unsafe {
            let foo: *mut importer::BIO = importer::BIO_new_file(lname.as_ptr(), lmode.as_ptr());
            if foo.is_null() {
                panic!("failed to read file");
            }
            let x509 = importer::PEM_read_bio_X509(foo, ptr::null(), 0, ptr::null());
            if x509.is_null() {
                 importer::BIO_free(foo);
                //panic!("failed to load file");
            }
        };
    }
}
