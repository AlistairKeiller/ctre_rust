use autocxx::prelude::*;

include_cpp! {
    #include "ctre/phoenix/motorcontrol/can/TalonFX.h"
    #include "ctre/phoenix/motorcontrol/ControlMode.h"
    #include "ctre/phoenix/unmanaged/Unmanaged.h"
    safety!(unsafe_ffi)
    generate!("ctre::phoenix::motorcontrol::can::TalonFX")
    generate!("ctre::phoenix::motorcontrol::TalonFXControlMode")
    generate!("ctre::phoenix::unmanaged::Unmanaged")
}

fn main() {
    cxx::let_cxx_string!(can = "can0");
    let mut falcon =
        ffi::ctre::phoenix::motorcontrol::can::TalonFX::new(c_int(0), &can).within_unique_ptr();
    for i in 0..10000 {
        falcon.pin_mut().Set(
            ffi::ctre::phoenix::motorcontrol::TalonFXControlMode::MotionMagic,
            i as f64*100.0,

        );

        ffi::ctre::phoenix::unmanaged::Unmanaged::FeedEnable(c_int(100));

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
