import time
import phoenix6

talon = phoenix6.hardware.TalonFX(0)
while(True):
    phoenix6.unmanaged.feed_enable(0.100)

    time.sleep(0.02)