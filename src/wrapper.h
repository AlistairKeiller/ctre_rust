#pragma once

#include <ctre/phoenix6/TalonFX.hpp>
#include <ctre/phoenix6/controls/DutyCycleOut.hpp>
#include <ctre/phoenix6/unmanaged/Unmanaged.hpp>
#include <ctre/phoenix6/controls/MotionMagicVoltage.hpp>

void configure_talonfx(int device_ID)
{
    ctre::phoenix6::configs::TalonFXConfiguration talonfx_config{};
    ctre::phoenix6::hardware::TalonFX talonfx{device_ID};

    talonfx_config.Slot0.kP = 4.8;
    talonfx_config.Slot0.kI = 0;
    talonfx_config.Slot0.kD = 0.1;

    talonfx_config.MotionMagic.MotionMagicCruiseVelocity = 5;
    talonfx_config.MotionMagic.MotionMagicAcceleration = 5;

    talonfx.GetConfigurator().Apply(talonfx_config);
}

void talonfx_motion_magic_voltage(int device_ID, double target_position)
{
    units::angle::turn_t target_position_turns{target_position};
    ctre::phoenix6::hardware::TalonFX talonfx{device_ID};
    ctre::phoenix6::controls::MotionMagicVoltage motion_magic_voltage{target_position_turns};
    talonfx.SetControl(motion_magic_voltage);
}

void talonfx_duty_cycle_out(int device_ID, double output)
{
    ctre::phoenix6::hardware::TalonFX talonfx{device_ID};
    ctre::phoenix6::controls::DutyCycleOut duty_cycle_out{output};
    talonfx.SetControl(duty_cycle_out);
}

void feed_enable() {
    ctre::phoenix::unmanaged::FeedEnable(100);
}