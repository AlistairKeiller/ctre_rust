use autocxx::prelude::*;

include_cpp! {
    #include "ctre/phoenix/motorcontrol/can/TalonFX.h"
    #include "ctre/phoenix/motorcontrol/ControlMode.h"
    #include "ctre/phoenix/unmanaged/Unmanaged.h"
    safety!(unsafe_ffi)
    generate!("ctre::phoenix::motorcontrol::can::TalonFX")
    generate_pod!("ctre::phoenix::motorcontrol::TalonFXControlMode")
    generate_pod!("ctre::phoenix::unmanaged::Unmanaged")
}

fn main() {
    cxx::let_cxx_string!(can = "can0");
    let mut falcon =
    ffi::ctre::phoenix::motorcontrol::can::TalonFX::new(c_int(0), &can).within_unique_ptr();
    
    // Config in tuner:
    // Motor Output.Neutral Deadband: 0
    // Motion Magic.Acceleration: 1000
    // Motion Magic.Cruise Velocity: 1000
    // Slot 0.kP: 0.6
    // Slot 0.kI: 0.001
    // Slot 0.kD: 6

    for i in 0..10000 {
        ffi::ctre::phoenix::unmanaged::Unmanaged::FeedEnable(c_int(100));
        
        falcon.pin_mut().Set(
            ffi::ctre::phoenix::motorcontrol::TalonFXControlMode::MotionMagic,
            (i * 100) as f64,
        );

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
