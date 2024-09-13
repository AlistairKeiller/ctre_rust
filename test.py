import time
import phoenix6

for _ in range(100):
    talon = phoenix6.hardware.TalonFX(1)
    talon_out = phoenix6.controls.DutyCycleOut(1.0)
    
    talon.set_control(talon_out)
    
    phoenix6.unmanaged.feed_enable(0.100)
    
    time.sleep(0.1)