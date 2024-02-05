

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

fn main() {
    unsafe {
        hello_for_static();
        hello_for_shared();
        hello_for_ccode();
    }
}
