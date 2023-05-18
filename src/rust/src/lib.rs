use extendr_api::prelude::*;

pub struct MyUsize {
    usize: usize,
}

#[extendr(r_class_name = "my_usize_r_name")]
impl MyUsize {
    pub fn new(x: f64) -> MyUsize {
        MyUsize { usize: x as usize }
    }
    pub fn print(&self) {
        println!("my_usize_r_name is {}", &self.usize);
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    impl my_usize_r_name;
}
