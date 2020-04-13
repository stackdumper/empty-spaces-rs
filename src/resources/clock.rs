use std::ops::{AddAssign, Div};
use std::thread;
use std::time::{Duration, Instant};

pub struct Clock {
    pub dt: f32,
    pub target_dt: f32,

    instant: Instant,
    duration: Duration,
}

impl Default for Clock {
    fn default() -> Self {
        Self::new(60)
    }
}

impl Clock {
    pub fn new(fps: u32) -> Self {
        let duration = Duration::from_secs(1).div(fps);
        let target_dt = duration.as_secs_f32();

        Self {
            dt: 0.0,
            instant: Instant::now(),
            target_dt,
            duration,
        }
    }

    pub fn tick(&mut self) {
        let elapsed = self.instant.elapsed();

        if elapsed < self.duration {
            self.dt = self.target_dt;
            thread::sleep(self.duration - elapsed);
        } else {
            self.dt = (elapsed - self.duration).as_secs_f32();
        }

        self.instant.add_assign(self.instant.elapsed());
    }
}
