use libm::{atan2, cos, exp, log, sin};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)] // Ensure PartialEq and Eq are derived
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    pub fn euler(r: f64, theta: f64) -> Self {
        Complex {
            real: r * cos(theta),
            imag: r * sin(theta),
        }
    }

    pub fn modulus(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn argument(&self) -> f64 {
        atan2(self.imag, self.real)
    }

    pub fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -1.0 * self.imag,
        }
    }

    pub fn square_root(&self) -> Complex {
        Complex {
            real: self.modulus().sqrt() * cos(self.argument() / 2.0),
            imag: self.modulus().sqrt() * sin(self.argument() / 2.0),
        }
    }

    pub fn logarithm(&self) -> Complex {
        Complex {
            real: log(self.modulus()),
            imag: self.argument(),
        }
    }

    pub fn exponent(&self) -> Complex {
        Complex {
            real: exp(self.real) * cos(self.imag),
            imag: exp(self.real) * sin(self.imag),
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{:.4} + {:.4}i", self.real, self.imag)
        } else {
            write!(f, "{:.4} - {:.4}i", self.real, -self.imag)
        }
    }
}

use std::ops::{Add, Div, Mul, Sub};

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Self::Output {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Self::Output {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Self::Output {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let denom = other.real * other.real + other.imag * other.imag;
        if denom == 0.0 {
            panic!("Attempt to divide by zero complex number");
        }
        Complex {
            real: (self.real * other.real + self.imag * other.imag) / denom,
            imag: (self.imag * other.real - self.real * other.imag) / denom,
        }
    }
}
