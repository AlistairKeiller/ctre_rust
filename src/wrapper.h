#pragma once

#include <ctre/phoenix6/TalonFX.hpp>
#include <ctre/phoenix6/controls/DutyCycleOut.hpp>
#include <ctre/phoenix6/controls/Follower.hpp>
#include <ctre/phoenix6/unmanaged/Unmanaged.hpp>
#include <ctre/phoenix6/controls/MotionMagicVoltage.hpp>

void configure_talonfx_follower(int device_ID, int leader_ID, bool oppose)
{
    ctre::phoenix6::hardware::TalonFX follower{device_ID};
    follower.SetControl(ctre::phoenix6::controls::Follower{leader_ID, oppose});
}

void configure_talonfx(int device_ID, double p, double i, double d, double cruise_velocity, double acceleration)
{
    ctre::phoenix6::configs::TalonFXConfiguration talonfx_config{};
    ctre::phoenix6::hardware::TalonFX talonfx{device_ID};

    talonfx_config.Slot0.kP = p;
    talonfx_config.Slot0.kI = i;
    talonfx_config.Slot0.kD = d;

    talonfx_config.MotionMagic.MotionMagicCruiseVelocity = cruise_velocity;
    talonfx_config.MotionMagic.MotionMagicAcceleration = acceleration;

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