use core::ops::{Add, Sub, Div, Mul};

use crate::util::{feq, Ray};

/// This structure conceptually represents a point or a vector on 
/// the 2-dimensional Cartesian plane. It may be vary on the context which represents which.
#[derive(Clone, Default, Debug, Copy, PartialEq, PartialOrd)]
pub struct Coordinate(pub(crate) f64, pub(crate) f64);

impl From<Coordinate> for (f64, f64){
    fn from(item: Coordinate) -> (f64, f64){
        (item.0, item.1)
    }
}

impl From<(f64, f64)> for Coordinate {
    fn from(item: (f64, f64)) -> Coordinate {
        Coordinate(item.0, item.1)
    }
}

impl From<geo_types::Coord<f64>> for Coordinate{
    fn from(value: geo_types::Coord<f64>) -> Self {
        Coordinate(value.x, value.y)
    }
}

impl From<Coordinate> for geo_types::Coord<f64>{
    fn from(value: Coordinate) -> geo_types::Coord<f64> {
        geo_types::geometry::Coord{x: value.0, y: value.1}
    }
}

impl Add for Coordinate{
    type Output = Self;
    fn add(self, rhs: Self) -> Self{
        Self(self.0+rhs.0, self.1+rhs.1)
    }
}

impl Sub for Coordinate{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self{
        Self(self.0-rhs.0, self.1-rhs.1)
    }
}

impl Mul<f64> for Coordinate{
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0*rhs, self.1*rhs)
    }
}

impl Div<f64> for Coordinate{
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0. {return self;}
        Self(self.0/rhs, self.1/rhs)
    }
}

impl Div<Coordinate> for Coordinate{
    type Output = f64;
    fn div(self, rhs: Self) -> Self::Output{
        if rhs.0 == 0. && rhs.1 == 0. {return 0.;}
        else if rhs.1 == 0. {return self.0/rhs.0;}
        else {return self.1/rhs.1;}
    }
}

impl Coordinate{
    /// Creates and returns a [Coordinate] w.r.t. the given argument.
    /// 
    /// # Argument
    /// 
    /// + x: 
    pub fn new(x: f64, y: f64) -> Self{
        Self{0: x, 1: y}
    }

    pub fn get_val(&self) -> (f64, f64){
        (self.0, self.1)
    }
    pub fn inner_product(&self, rhs: &Self) -> f64{
        self.0*rhs.0 + self.1*rhs.1
    }

    pub fn outer_product(&self, rhs: &Self) -> f64{
        self.0*rhs.1-self.1*rhs.0
    }

    pub fn norm(&self) -> f64{
        self.inner_product(self).sqrt()
    }

    pub fn dist_coord(&self, rhs: &Coordinate) -> f64{
        f64::sqrt((self.0-rhs.0)*(self.0-rhs.0) + (self.1-rhs.1)*(self.1-rhs.1))
    }
    
    pub fn dist_ray(&self, rhs: &Ray) -> f64{
        if rhs.is_degenerated() {return self.dist_coord(&rhs.origin);}
        return f64::abs((*self-rhs.origin).outer_product(&rhs.angle)) / rhs.angle.norm();
    }

    pub fn eq(&self, rhs: &Self) -> bool{
        feq(self.0, rhs.0) && feq(self.1, rhs.1)
    }
}