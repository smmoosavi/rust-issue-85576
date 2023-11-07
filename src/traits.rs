mod systick;

pub trait FromTime<U> {
    fn from_ns(nanos: U) -> Self;

    fn from_us(micros: U) -> Self;

    fn from_ms(millis: U) -> Self;

    fn from_secs(secs: U) -> Self;

    fn from_minutes(minutes: U) -> Self;

    fn from_hours(hours: U) -> Self;
}

pub trait TimeInto<U> {
    fn ns_into(self) -> U;

    fn us_into(self) -> U;

    fn ms_into(self) -> U;

    fn secs_into(self) -> U;

    fn minutes_into(self) -> U;

    fn hours_into(self) -> U;
}

pub trait ToTime<U> {
    fn to_ns(&self) -> U;

    fn to_us(&self) -> U;

    fn to_ms(&self) -> U;

    fn to_secs(&self) -> U;

    fn to_minutes(&self) -> U;

    fn to_hours(&self) -> U;
}

impl<U, T> TimeInto<U> for T
where
    U: FromTime<T>,
{
    fn ns_into(self) -> U {
        FromTime::from_ns(self)
    }

    fn us_into(self) -> U {
        FromTime::from_us(self)
    }

    fn ms_into(self) -> U {
        FromTime::from_ms(self)
    }

    fn secs_into(self) -> U {
        FromTime::from_secs(self)
    }

    fn minutes_into(self) -> U {
        FromTime::from_minutes(self)
    }

    fn hours_into(self) -> U {
        FromTime::from_hours(self)
    }
}
