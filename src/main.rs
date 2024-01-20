

#[link(name = "hello")]
extern "C" {

    pub fn hello_for_static();
    pub fn hello_for_dylib();
}

fn main() {
    unsafe {
        hello_for_static();
        hello_for_dylib();
    }
}
