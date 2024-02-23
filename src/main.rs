// #[link(name = "hello_static")]
// extern "C" {
//     pub fn hello_for_static();
// }

// #[link(name = "cpp_static", kind = "static")]
// extern "C" {
//     pub fn hello_for_cpp() -> *mut std::os::raw::c_char;
// }
//
// // #[link(name = "hello_shared")]
// // extern "C" {
// //     pub fn hello_for_shared();
// // }
// //
// #[link(name = "hello_for_ccode")]
// extern "C" {
//     pub fn hello_for_ccode()->*const std::os::raw::c_char;
// }
//
// #[link(name = "cpp_shared")]
// extern "C" {
//     //const char* get_hello(char *error);
//     //to rust code
//     pub fn get_hello(error: *mut std::os::raw::c_char) -> *const std::os::raw::c_char;
// }

// #[link(name = "zsign")]
// extern "C" {
    //const char* get_hello(char *error);
    //to rust code
    // pub fn sign_ipa(ipa_path: *std::os::raw::c_char,key_path: *std::os::raw::c_char) -> *const std::os::raw::c_char;
    // void sign_ipa(char *ipa_path, char *key_path, char *mp_path, char *dylib_file_path, char *icon_path, char *tmp_folder_path);
    // pub fn sign_ipa(ipa_path: *const std::os::raw::c_char, key_path: *const std::os::raw::c_char, mp_path: *const std::os::raw::c_char, dylib_file_path: *const std::os::raw::c_char, icon_path: *const std::os::raw::c_char, tmp_folder_path: *const std::os::raw::c_char, error: *mut std::os::raw::c_char) -> *const std::os::raw::c_int;
// }

use zsign;

fn main() {
    unsafe {
        // let c_str = hello_for_ccode();
        // let r_str = std::ffi::CStr::from_ptr(c_str).to_string_lossy();
        // println!("hello_for_ccode: {}", r_str);
        // hello_for_ccode();
        // let c_str = hello_for_cpp();
        // let r_str = std::ffi::CStr::from_ptr(c_str).to_string_lossy();
        // println!("hello_for_cpp: {}", r_str);
        // let r_str = std::ffi::CStr::from_ptr(c_str).to_string_lossy();
        // println!("hello_for_cpp: {}", r_str);
        // hello_for_ccode();
        // string keyPath = "/Users/lake/dounine/github/rust/rust-sign/ipa/key.pem";
        // string mpPath = "/Users/lake/dounine/github/rust/rust-sign/ipa/lake_13_pm.mobileprovision";
        // string ipaPath = "/Users/lake/dounine/github/rust/rust-sign/ipa/video.ipa";
        // string dylibFilePath = "/Users/lake/dounine/github/rust/rust-sign/ipa/d.dylib";
        // string iconPath = "/Users/lake/dounine/github/rust/rust-sign/ipa/icon.png";
        // string tmpFolderPath = "/Users/lake/dounine/github/rust/rust-sign/ipa/tmp";
        //
        // hello_for_static();

        let ipa_path = std::ffi::CString::new("./ipa/video.ipa").unwrap();
        let key_path = std::ffi::CString::new("./ipa/key.pem").unwrap();
        let mp_path = std::ffi::CString::new("./ipa/lake_13_pm.mobileprovision").unwrap();
        let dylib_file_path = std::ffi::CString::new("./ipa/d.dylib").unwrap();
        let icon_path = std::ffi::CString::new("./ipa/icon.png").unwrap();
        let tmp_folder_path = std::ffi::CString::new("./ipa/tmp").unwrap();

        let mut error_mut: [std::os::raw::c_char; 1024] = [0; 1024];
        // let c_error = get_hello(error_mut.as_mut_ptr());
        zsign::sign_ipa(ipa_path.as_ptr(), key_path.as_ptr(), mp_path.as_ptr(), dylib_file_path.as_ptr(), icon_path.as_ptr(), tmp_folder_path.as_ptr(), error_mut.as_mut_ptr());
        // let error_str = std::ffi::CStr::from_ptr(error_mut.as_ptr()).to_string_lossy();
        // println!("error_mut: {}", error_str);
        // hello_for_shared();
        // hello_for_ccode();
        //
        // //第一个c+++代码
        // let mut error_mut: [std::os::raw::c_char; 1024] = [0; 1024];
        // let c_str = get_hello(error_mut.as_mut_ptr());
        // let r_str = unsafe { std::ffi::CStr::from_ptr(c_str).to_string_lossy() };
        // let r_error_str = unsafe { std::ffi::CStr::from_ptr(error_mut.as_ptr()).to_string_lossy() };
        // println!("get_hello: {}", r_str);
        // println!("error_mut: {}", r_error_str);
    }
}
