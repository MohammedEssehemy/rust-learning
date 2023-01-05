// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const SECOND_IN_EARTH_YEAR: f64 = 31557600_f64;

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            earth_years: s as f64 / SECOND_IN_EARTH_YEAR,
        }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::ORBITAL_PERIOD
    }
}

macro_rules! planet {
    ($planet_name:ident, $orbital_period:expr) => {
        pub struct $planet_name;
        impl Planet for $planet_name {
            const ORBITAL_PERIOD: f64 = $orbital_period;
        }
    };
}

planet!(Mercury, 0.2408467_f64);
planet!(Venus, 0.61519726_f64);
planet!(Earth, 1_f64);
planet!(Mars, 1.8808158_f64);
planet!(Jupiter, 11.862615_f64);
planet!(Saturn, 29.447498_f64);
planet!(Uranus, 84.016846_f64);
planet!(Neptune, 164.79132_f64);
