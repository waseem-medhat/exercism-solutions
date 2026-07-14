#[derive(Debug)]
pub struct Duration {
    earth_seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { earth_seconds: s }
    }
}

pub trait Planet {
    const FACTOR: f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_seconds as f64 / Self::FACTOR / 60.0 / 60.0 / 24.0 / 365.25
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const FACTOR: f64 = 0.2408467;
}
impl Planet for Venus {
    const FACTOR: f64 = 0.61519726;
}
impl Planet for Earth {
    const FACTOR: f64 = 1.0;
}
impl Planet for Mars {
    const FACTOR: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const FACTOR: f64 = 11.862615;
}
impl Planet for Saturn {
    const FACTOR: f64 = 29.447498;
}
impl Planet for Uranus {
    const FACTOR: f64 = 84.016846;
}
impl Planet for Neptune {
    const FACTOR: f64 = 164.79132;
}
