mod systick;

pub trait FromTime<U> {
    fn from_ms(millis: U) -> Self;
}

pub trait TimeInto<U> {
    fn ms_into(self) -> U;
}

pub trait ToTime<U> {
    fn to_ms(&self) -> U;
}

impl<U, T> TimeInto<U> for T
where
    U: FromTime<T>,
{
    fn ms_into(self) -> U {
        FromTime::from_ms(self)
    }
}
