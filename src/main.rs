use autocxx::prelude::*;

include_cpp! {
    #include "wrapper.h"
    safety!(unsafe_ffi)
    generate!("talonfx_duty_cycle_out")
    generate!("talonfx_motion_magic_voltage")
    generate!("configure_talonfx")
    generate!("feed_enable")
}

fn main() {
    ffi::configure_talonfx(c_int(0));

    let mut position = 0.0;

    loop {
        ffi::talonfx_motion_magic_voltage(c_int(0), position);
        position += 0.1;
        ffi::feed_enable();

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
