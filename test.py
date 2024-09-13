import time
import phoenix6

for _ in range(100):
    talon = phoenix6.hardware.TalonFX(0)
    talon_out = phoenix6.controls.DutyCycleOut(0.1)
    
    talon.set_control(talon_out)
    
    phoenix6.unmanaged.feed_enable(0.100)
    
    time.sleep(0.1)