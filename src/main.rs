#[link(name = "hello_static")]
extern "C" {
    pub fn hello_for_static();
}

#[link(name = "hello_shared")]
extern "C" {
    pub fn hello_for_shared();
}

extern "C" {
    pub fn hello_for_ccode();
}

#[link(name = "cpp_shared")]
extern "C" {
    //const char* get_hello(char *error);
    //to rust code
    pub fn get_hello(error: *mut std::os::raw::c_char) -> *const std::os::raw::c_char;
}

fn main() {
    unsafe {
        hello_for_static();
        hello_for_shared();
        hello_for_ccode();

        //第一个c+++代码
        let mut error_mut: [std::os::raw::c_char; 1024] = [0; 1024];
        let c_str = get_hello(error_mut.as_mut_ptr());
        let r_str = unsafe { std::ffi::CStr::from_ptr(c_str).to_string_lossy() };
        let r_error_str = unsafe { std::ffi::CStr::from_ptr(error_mut.as_ptr()).to_string_lossy() };
        println!("get_hello: {}", r_str);
        println!("error_mut: {}", r_error_str);
    }
}
