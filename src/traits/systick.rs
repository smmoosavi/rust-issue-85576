use rtic_monotonics::systick::ExtU32;

use crate::FromTime;

// todo: incorrectly rust compiler complains about this, so we have to use the fugit types directly
// error[E0119]: conflicting implementations of trait `from_time::ToTime<u32>`

use rtic_monotonics::systick::Systick;
use rtic_monotonics::Monotonic;
#[allow(dead_code)]
type SystickDuration = <Systick as Monotonic>::Duration;
#[allow(dead_code)]
type SystickInstant = <Systick as Monotonic>::Instant;

use rtic_monotonics::systick::fugit;
const TIMER_HZ: u32 = 1_000;
#[allow(dead_code)]
type FugitInstant = fugit::TimerInstantU32<TIMER_HZ>;
#[allow(dead_code)]
type FugitDuration = fugit::TimerDurationU32<TIMER_HZ>;

type Duration = FugitDuration;
type Instant = FugitInstant;

// type Duration = SystickDuration;
// type Instant = SystickInstant;

impl FromTime<u32> for Duration {
    fn from_ms(millis: u32) -> Self {
        millis.millis()
    }
}

impl FromTime<u32> for Instant {
    fn from_ms(millis: u32) -> Self {
        Self::from_ticks(millis)
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use super::*;

    #[test]
    fn test_type_ids() {
        assert_eq!(
            TypeId::of::<SystickDuration>(),
            TypeId::of::<FugitDuration>()
        );
        assert_eq!(TypeId::of::<SystickInstant>(), TypeId::of::<FugitInstant>());

        assert_ne!(
            TypeId::of::<SystickDuration>(),
            TypeId::of::<SystickInstant>()
        );
        assert_ne!(TypeId::of::<FugitDuration>(), TypeId::of::<FugitInstant>());
    }
}
