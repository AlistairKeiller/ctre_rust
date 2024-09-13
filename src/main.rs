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
        ffi::ctre::phoenix::motorcontrol::can::TalonFX::new(c_int(1), &can).within_unique_ptr();
    for _ in 0..50 {
        falcon.pin_mut().Set(
            ffi::ctre::phoenix::motorcontrol::TalonFXControlMode::PercentOutput,
            0.5,
        );

        ffi::ctre::phoenix::unmanaged::Unmanaged::FeedEnable(c_int(100));

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
