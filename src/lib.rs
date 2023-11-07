mod internal;

pub trait FromTime {}

#[allow(dead_code)]
type CrateDuration = <internal::Systick as internal::Monotonic>::Duration;
#[allow(dead_code)]
type CrateInstant = <internal::Systick as internal::Monotonic>::Instant;

#[allow(dead_code)]
type OtherDuration = <other::Systick as other::Monotonic>::Duration;
#[allow(dead_code)]
type OtherInstant = <other::Systick as other::Monotonic>::Instant;

#[cfg(feature = "crate-direct")]
mod test_crate_direct {
    use super::*;

    impl FromTime for internal::Duration {}

    impl FromTime for internal::Instant {}
}

#[cfg(feature = "crate-systick")]
mod test_crate_systick {
    use super::*;

    impl FromTime for CrateDuration {}

    impl FromTime for CrateInstant {}
}

#[cfg(feature = "other-direct")]
mod test_other_direct {
    use super::*;

    impl FromTime for other::Duration {}

    impl FromTime for other::Instant {}
}

#[cfg(feature = "other-systick")]
mod test_other_systick {
    use super::*;

    impl FromTime for OtherDuration {}

    impl FromTime for OtherInstant {}
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use super::*;

    #[test]
    fn test_type_ids() {
        assert_eq!(
            TypeId::of::<OtherDuration>(),
            TypeId::of::<other::Duration>()
        );
        assert_eq!(TypeId::of::<OtherInstant>(), TypeId::of::<other::Instant>());

        assert_ne!(TypeId::of::<OtherDuration>(), TypeId::of::<OtherInstant>());
    }
}
