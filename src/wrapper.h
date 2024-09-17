#pragma once

#include <phoenix6/ctre/phoenix6/controls/DutyCycleOut.hpp>

class DutyCycleOutWrapper : public ctre::phoenix6::controls::DutyCycleOut
{
public:
    DutyCycleOutWrapper(double Output) : DutyCycleOut{Output}
    {
    }

    void WithOutput(double Output)
    {
        ctre::phoenix6::controls::DutyCycleOut::WithOutput(Output);
    }
};

class TalonFXWrapper : public ctre::phoenix6::hardware::TalonFX
{
public:
    TalonFXWrapper(int DeviceID, std::string canbus) : TalonFX{DeviceID, canbus}
    {
    }

    ctre::phoenix::StatusCode SetControl(DutyCycleOutWrapper &request)
    {
        return ctre::phoenix6::hardware::TalonFX::SetControl(request);
    }
};