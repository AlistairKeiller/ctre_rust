use autocxx::prelude::*;

include_cpp! {
    // #include "ctre/phoenix6/unmanaged/Unmanaged.hpp"
    // #include "ctre/phoenix6/TalonFX.hpp"
    #include "wrapper.h"
    safety!(unsafe_ffi)
    // generate!("ctre::phoenix6::hardware::TalonFX")
    // generate!("ctre::phoenix::unmanaged::FeedEnable")
    generate!("DutyCycleOutWrapper")
}

fn main() {
    // let falcon = ffi::ctre::phoenix6::hardware::TalonFX::new(c_int(0), "can0").within_unique_ptr();

    // let x = ffi::DutyCycleOutWrapper::new(0.5);

    for _ in 0..10000 {
        // ffi::ctre::phoenix::unmanaged::FeedEnable(c_int(100));

        falcon.pin_mut().SetControl(falcon.as_ref());

        // std::thread::sleep(std::time::Duration::from_millis(20));
    }
}