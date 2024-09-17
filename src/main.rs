use autocxx::prelude::*;

include_cpp! {
    #include "wrapper.h"
    safety!(unsafe_ffi)
    generate!("run_talonfx")
    generate!("feed_enable")
}

fn main() {
    loop {
        ffi::run_talonfx(c_int(0), 0.1);
        ffi::feed_enable();

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
