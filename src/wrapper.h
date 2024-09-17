#pragma once

#include <ctre/phoenix6/TalonFX.hpp>
#include <ctre/phoenix6/controls/DutyCycleOut.hpp>
#include <ctre/phoenix6/unmanaged/Unmanaged.hpp>

void run_talonfx(int DeviceID, double Output)
{
    ctre::phoenix6::hardware::TalonFX talonfx(DeviceID);
    ctre::phoenix6::controls::DutyCycleOut dutyCycleOut(Output);
    talonfx.SetContxrol(dutyCycleOut);
}

void feed_enable() {
    ctre::phoenix::unmanaged::FeedEnable(100);
}