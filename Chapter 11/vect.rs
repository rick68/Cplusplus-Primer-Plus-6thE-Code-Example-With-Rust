// vect.rs -- methods for the Vector struct

use std::f64;
use std::default::Default;
use std::ops::{Add, Drop, Mul, Neg, Sub};
use std::fmt::{Display, Formatter, Result};

// compute degrees in one radian
const RAD_TO_DEG: f64 = 180.0 / f64::consts::PI;
// should be about 57.2957795130823

// RECT for rectangular, POL for Polar modes
#[derive(Clone)]
pub enum Mode {
    RECT,
    POL,
}

#[derive(Clone)]
pub struct Vector {
    x: f64,     // horizontal value
    y: f64,     // vertical value
    mag: f64,   // length of vector
    ang: f64,   // direction of vector in degrees
    mode: Mode, // RECT or POL
}

impl Default for Vector {
    // default constructor
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            mag: 0.0,
            ang: 0.0,
            mode: Mode::RECT,
        }
    }
}

impl Drop for Vector {
    // destructor
    fn drop(&mut self) {
    }
}

macro_rules! Vector_new {
    ($n1:expr, $n2:expr, $form:expr) => (
        Vector::new($n1, $n2, $form)
    );
    ($n1:expr, $n2:expr) => (
        Vector::new_with_rect($n1, $n2)
    );
    () => (
        Vector::default()
    );
}

macro_rules! Vector_reset {
    ($vector:expr, $n1:expr, $n2:expr, $form:expr) => (
        $vector.reset($n1, $n2, $form)
    );
    ($vector:expr, $n1:expr, $n2:expr) => (
        $vector.reset_with_rect($n1, $n2)
    );
}

impl Vector {
    // construct vector from rectangular coordinates if form is r
    // (the default) or else from polar coordinates if form is p
    pub fn new(n1: f64, n2: f64, form: Mode) -> Self {
        match form {
            Mode::RECT => {
                let mut obj: Self = Self {
                    x: n1,
                    y: n2,
                    mag: 0.0,
                    ang: 0.0,
                    mode: Mode::RECT,
                };
                obj.set_mag();
                obj.set_ang();
                obj
            },
            Mode::POL => {
                let mut obj: Self = Self {
                    x: 0.0,
                    y: 0.0,
                    mag: n1,
                    ang: n2,
                    mode: Mode::POL,
                };
                obj.set_x();
                obj.set_y();
                obj
            },
        }
    }

    #[inline]
    pub fn new_with_rect(n1: f64, n2: f64) -> Self {
        Vector::new(n1, n2, Mode::RECT)
    }

    // reset vector from rectangular coordinates if form is
    // RECT (the default) or else from polar coordinates if
    // form is POL
    pub fn reset(&mut self, n1: f64, n2: f64, form: Mode) {
        match form {
            Mode::RECT => {
                self.x = n1;
                self.y = n2;
                self.set_mag();
                self.set_ang();
            },
            Mode::POL => {
                self.mag = n1;
                self.ang = n2 / RAD_TO_DEG;
                self.set_x();
                self.set_y();
            },
        }
        self.mode = form;
    }

    #[inline]
    pub fn reset_with_rect(&mut self, n1: f64, n2: f64) {
        self.reset(n1, n2, Mode::RECT)
    }

    // report x value
    pub fn xval(&self) -> f64 {
        self.x
    }

    // report y value
    pub fn yval(&self) -> f64 {
        self.y
    }

    // report magnitude
    pub fn magval(&self) -> f64 {
        self.mag
    }

    // report angle
    pub fn angval(&self) -> f64 {
        self.ang
    }

    // set to polar mode
    pub fn polar_mode(&mut self) {
        self.mode = Mode::POL;
    }

    // set to rectangular mode
    pub fn rect_mode(&mut self) {
        self.mode = Mode::RECT;
    }


    fn set_mag(&mut self) {
        self.mag = self.x * f64::sqrt(self.x * self.x + self.y * self.y);
    }

    fn set_ang(&mut self) {
        if self.x == 0.0 && self.y == 0.0 {
            self.ang = 0.0;
        } else {
            self.ang = f64::atan2(self.y, self.x);
        }
    }

    fn set_x(&mut self) {
        self.x = self.mag * f64::cos(self.ang);
    }

    fn set_y(&mut self) {
        self.y = self.mag * f64::sin(self.ang);
    }
}

// operator+ overloading
// add tow Vectors
impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new_with_rect(self.x + other.x, self.y + other.y)
    }
}

// operator- overloading
// rubtract Vector b from a
impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new_with_rect(self.x - other.x, self.y - other.y)
    }
}

// operator-() overloading
// reverse sign of Vector
impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new_with_rect(-self.x, -self.y)
    }
}

// operator* overloading
// multiply vector by rhs
impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector::new_with_rect(self.x * rhs, self.y * rhs)
    }
}
// multiply f64 variable by Vector rhs
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector::new_with_rect(self * rhs.x, self * rhs.y)
    }
}

// display rectangular coordinates if mode is RECT,
// else display polar coordinates if mode is POL
impl Display for Vector {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.mode {
            Mode::RECT => {
                write!(f, "(x,y) = ({}, {})", self.x, self.y)
            },
            Mode::POL => {
                write!(f, "(m,a) = ({}, {})", self.mag, self.ang * RAD_TO_DEG)
            },
        }
    }
}
