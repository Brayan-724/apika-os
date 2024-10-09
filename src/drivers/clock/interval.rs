pub struct Interval<T: FnMut()>(T, IntervalClock);

impl<T: FnMut()> Interval<T> {
    pub fn new(interval: usize, fun: T) -> Self {
        Self(fun, IntervalClock::new(interval))
    }

    pub fn update(&mut self) {
        if self.1.update() {
            self.0();
        }
    }
}

pub struct IntervalClock(usize, usize);

impl IntervalClock {
    pub fn new(interval: usize) -> Self {
        Self(interval, 0)
    }

    pub fn update(&mut self) -> bool {
        if self.1 == self.0 {
            self.1 = 0;
            true
        } else {
            self.1 += 1;
            false
        }
    }
}
