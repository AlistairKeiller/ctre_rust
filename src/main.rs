use autocxx::prelude::*;

include_cpp! {
    #include "ctre/phoenix6/unmanaged/Unmanaged.hpp"
    #include "ctre/phoenix6/TalonFX.hpp"
    #include "wrapper.h"
    safety!(unsafe_ffi)
    generate!("ctre::phoenix6::hardware::TalonFX")
    generate!("ctre::phoenix::unmanaged::FeedEnable")
    generate!("DutyCycleOutWrapper")
    generate!("TalonFXWrapper")
}

fn main() {
    let mut falcon = ffi::TalonFXWrapper::new(c_int(0), "can0").within_unique_ptr();

    let mut duty = ffi::DutyCycleOutWrapper::new(0.5).within_unique_ptr();

    for _ in 0..10000 {
        let _ = ffi::ctre::phoenix::unmanaged::FeedEnable(c_int(100));

        let _ = falcon.pin_mut().SetControl(duty.pin_mut());

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
