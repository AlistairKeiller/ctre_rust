#pragma once

#include <phoenix6/ctre/phoenix6/controls/DutyCycleOut.hpp>

class DutyCycleOutWrapper : public ctre::phoenix6::controls::DutyCycleOut
{
public:
    DutyCycleOutWrapper(double Output) : ctre::phoenix6::controls::DutyCycleOut(Output)
    {
    }

    void WithOutput(double Output) {
        ctre::phoenix6::controls::DutyCycleOut::WithOutput(Output);
    }
};

class TalonFXWrapper : public ctre::phoenix6::hardware::TalonFX
{
public:
    TalonFXWrapper(int DeviceID) : ctre::phoenix6::hardware::TalonFX(DeviceID)
    {
    }
    ctre::phoenix::StatusCode SetControl(DutyCycleOutWrapper& request) {
        return ctre::phoenix6::hardware::TalonFX::SetControl(request);
    }
};