from pyb import Timer
import micropython

micropython.alloc_emergency_exception_buf(1000)


class SignalPattern:
    def __init__(self, pattern: str):
        self.pattern = pattern
        self.current_index = 0

    def next_bit(self):
        if self.current_index == len(self.pattern):
            self.current_index = 0
            return None
        else:
            bit = int(self.pattern[self.current_index])
            self.current_index += 1
            return bit


def poll(_timer: Timer):
    pyb.LED(1).toggle()

    signal_timer.init(freq=4)
    signal_timer.callback(signal)


def signal(timer: Timer):
    led = pyb.LED(4)
    bit = signal_pattern.next_bit()
    if bit is None:
        led.on()
        timer.deinit()
    elif bit == 0:
        led.off()
    elif bit == 1:
        led.on()
    else:
        raise ValueError


polling_timer = Timer(1, freq=.4)
signal_timer = Timer(2)
signal_pattern = SignalPattern("01011")
polling_timer.callback(poll)
