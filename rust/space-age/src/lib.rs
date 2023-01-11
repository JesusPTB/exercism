// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            earth_years: s as f64 / 31_557_600_f64,
        }
    }
}

pub trait Planet {
    fn get_from_earth() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::get_from_earth()
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
    fn get_from_earth() -> f64 {
        0.240_846_7
    }
}

impl Planet for Venus {
    fn get_from_earth() -> f64 {
        0.615_197_26
    }
}

impl Planet for Earth {
    fn get_from_earth() -> f64 {
        1.0
    }
}

impl Planet for Mars {
    fn get_from_earth() -> f64 {
        1.880_815_8
    }
}

impl Planet for Jupiter {
    fn get_from_earth() -> f64 {
        11.862_615
    }
}

impl Planet for Saturn {
    fn get_from_earth() -> f64 {
        29.447_498
    }
}

impl Planet for Uranus {
    fn get_from_earth() -> f64 {
        84.016_846
    }
}

impl Planet for Neptune {
    fn get_from_earth() -> f64 {
        164.791_32
    }
}