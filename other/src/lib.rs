pub trait Monotonic {
    type Duration;
    type Instant;
}

pub struct Systick;
pub struct Duration;
pub struct Instant;

impl Monotonic for Systick {
    type Duration = Duration;
    type Instant = Instant;
}
