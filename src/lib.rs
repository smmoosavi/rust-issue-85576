mod internal;
pub trait FromTime {}

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

#[cfg(feature = "fugit")]
type Duration = FugitDuration;
#[cfg(feature = "fugit")]
type Instant = FugitInstant;

#[cfg(feature = "systick")]
type Duration = SystickDuration;
#[cfg(feature = "systick")]
type Instant = SystickInstant;

// direct from internal
#[cfg(feature = "direct")]
type Duration = internal::Duration;
#[cfg(feature = "direct")]
type Instant = internal::Instant;

// from internal Monotonic
#[cfg(feature = "internal-systick")]
type Duration = <internal::Systick as internal::Monotonic>::Duration;
#[cfg(feature = "internal-systick")]
type Instant = <internal::Systick as internal::Monotonic>::Instant;

// direct from other
#[cfg(feature = "other-direct")]
type Duration = other::Duration;
#[cfg(feature = "other-direct")]
type Instant = other::Instant;

// from other Monotonic
#[cfg(feature = "other-systick")]
type Duration = <other::Systick as other::Monotonic>::Duration;
#[cfg(feature = "other-systick")]
type Instant = <other::Systick as other::Monotonic>::Instant;

impl FromTime for Duration {}

impl FromTime for Instant {}

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
