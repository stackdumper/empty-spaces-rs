use std::ops::{AddAssign, Div};
use std::thread;
use std::time::{Duration, Instant};

/// Clock is used to keep track of time in the game world
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
    /// create new Clock instance
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

    // update clock, pause thread to match desired fps
    pub fn tick(&mut self) {
        let elapsed = self.instant.elapsed();

        println!("{} - {}", self.target_dt, elapsed.as_secs_f32());

        if elapsed < self.duration {
            thread::sleep(self.duration - elapsed);
            self.dt = self.instant.elapsed().as_secs_f32();
        } else {
            self.dt = elapsed.as_secs_f32();
        }

        self.instant.add_assign(self.instant.elapsed());
    }
}
