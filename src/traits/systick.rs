use rtic_monotonics::systick::ExtU32;

use crate::FromTime;
use crate::ToTime;

// todo: incorrectly rust compiler complains about this, so we have to use the fugit types directly
// error[E0119]: conflicting implementations of trait `from_time::ToTime<u32>`

use rtic_monotonics::systick::Systick;
use rtic_monotonics::Monotonic;
type Duration = <Systick as Monotonic>::Duration;
type Instant = <Systick as Monotonic>::Instant;

// use rtic_monotonics::systick::fugit;
// const TIMER_HZ: u32 = 1_000;
// type Instant = fugit::TimerInstantU32<TIMER_HZ>;
// type Duration = fugit::TimerDurationU32<TIMER_HZ>;

impl FromTime<u32> for Duration {
    fn from_ms(millis: u32) -> Self {
        millis.millis()
    }
}

impl ToTime<u32> for Duration {
    fn to_ms(&self) -> u32 {
        self.to_millis()
    }
}

impl FromTime<u32> for Instant {
    fn from_ms(millis: u32) -> Self {
        Self::from_ticks(millis)
    }
}

impl ToTime<u32> for Instant {
    fn to_ms(&self) -> u32 {
        self.ticks()
    }
}
