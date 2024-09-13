import time
import phoenix6

while(True):
    talon = phoenix6.hardware.TalonFX(0)
    
    phoenix6.unmanaged.feed_enable(0.100)

    time.sleep(0.1)