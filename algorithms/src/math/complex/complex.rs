use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Polar {
    pub magnitude: f64,
    pub phase: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }

    pub fn to_polar(self) -> Polar {
        Polar {
            magnitude: self.magnitude(),
            phase: self.phase(),
        }
    }

    pub fn from_polar(p: Polar) -> Complex {
        Complex {
            real: p.magnitude * p.phase.cos(),
            imag: p.magnitude * p.phase.sin(),
        }
    }

    pub fn conjugate(self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }

    pub fn magnitude(self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn phase(self) -> f64 {
        self.imag.atan2(self.real)
    }
}

impl Polar {
    pub fn new(magnitude: f64, phase: f64) -> Polar {
        Polar { magnitude, phase }
    }

    pub fn conjugate(self) -> Polar {
        Polar {
            magnitude: self.magnitude,
            phase: -self.phase,
        }
    }

    pub fn to_complex(self) -> Complex {
        Complex {
            real: self.magnitude * self.phase.cos(),
            imag: self.magnitude * self.phase.sin(),
        }
    }

    pub fn from_complex(z: Complex) -> Polar {
        Polar {
            magnitude: z.magnitude(),
            phase: z.phase(),
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
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
        Complex {
            real: (self.real * other.real + self.imag * other.imag) / denom,
            imag: (self.imag * other.real - self.real * other.imag) / denom,
        }
    }
}

impl Add for Polar {
    type Output = Polar;

    fn add(self, other: Polar) -> Polar {
        Polar::from_complex(self.to_complex() + other.to_complex())
    }
}

impl Sub for Polar {
    type Output = Polar;

    fn sub(self, other: Polar) -> Polar {
        Polar::from_complex(self.to_complex() - other.to_complex())
    }
}

impl Mul for Polar {
    type Output = Polar;

    fn mul(self, other: Polar) -> Polar {
        Polar::new(self.magnitude * other.magnitude, self.phase + other.phase)
    }
}

impl Div for Polar {
    type Output = Polar;

    fn div(self, other: Polar) -> Polar {
        Polar::new(self.magnitude / other.magnitude, self.phase - other.phase)
    }
}
