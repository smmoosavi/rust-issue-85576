mod systick;

pub trait FromTime<U> {
    fn from_ms(millis: U) -> Self;
}
