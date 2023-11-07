use rtic_monotonics::systick::ExtU32;

use crate::FromTime;
use crate::ToTime;

// todo: incorrectly rust compiler complains about this, so we have to use the fugit types directly
// error[E0119]: conflicting implementations of trait `from_time::ToTime<u32>`
// use rtic_monotonics::systick::Systick;
// use rtic_monotonics::Monotonic;
// type Duration = <Systick as Monotonic>::Duration;
// type Instant = <Systick as Monotonic>::Instant;

use rtic_monotonics::systick::fugit;
const TIMER_HZ: u32 = 1_000;
type Instant = fugit::TimerInstantU32<TIMER_HZ>;
type Duration = fugit::TimerDurationU32<TIMER_HZ>;

impl FromTime<u32> for Duration {
    fn from_ns(nanos: u32) -> Self {
        nanos.nanos()
    }

    fn from_us(micros: u32) -> Self {
        micros.micros()
    }

    fn from_ms(millis: u32) -> Self {
        millis.millis()
    }

    fn from_secs(secs: u32) -> Self {
        secs.secs()
    }

    fn from_minutes(minutes: u32) -> Self {
        minutes.minutes()
    }

    fn from_hours(hours: u32) -> Self {
        hours.hours()
    }
}

impl ToTime<u32> for Duration {
    fn to_ns(&self) -> u32 {
        self.to_nanos()
    }

    fn to_us(&self) -> u32 {
        self.to_micros()
    }

    fn to_ms(&self) -> u32 {
        self.to_millis()
    }

    fn to_secs(&self) -> u32 {
        self.to_secs()
    }

    fn to_minutes(&self) -> u32 {
        self.to_minutes()
    }

    fn to_hours(&self) -> u32 {
        self.to_hours()
    }
}

impl FromTime<u32> for Instant {
    fn from_ns(nanos: u32) -> Self {
        Self::from_us(nanos / 1000)
    }

    fn from_us(micros: u32) -> Self {
        Self::from_ms(micros / 1000)
    }

    fn from_ms(millis: u32) -> Self {
        Self::from_ticks(millis)
    }

    fn from_secs(secs: u32) -> Self {
        Self::from_ms(secs * 1000)
    }

    fn from_minutes(minutes: u32) -> Self {
        Self::from_secs(minutes * 60)
    }

    fn from_hours(hours: u32) -> Self {
        Self::from_minutes(hours * 60)
    }
}

impl ToTime<u32> for Instant {
    fn to_ns(&self) -> u32 {
        self.to_us() * 1000
    }

    fn to_us(&self) -> u32 {
        self.to_ms() * 1000
    }

    fn to_ms(&self) -> u32 {
        self.ticks()
    }

    fn to_secs(&self) -> u32 {
        self.to_ms() / 1000
    }

    fn to_minutes(&self) -> u32 {
        self.to_secs() / 60
    }

    fn to_hours(&self) -> u32 {
        self.to_minutes() / 60
    }
}
