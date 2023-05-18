//! This module provides a conceptual structure of points and half-lines.
//! 
//! See more details on each item.

mod coordinate;
mod ray;

pub use coordinate::Coordinate;
pub use ray::Ray;

const EPS: f64 = 1e-9;

pub(crate) fn feq(x: f64, y: f64) -> bool{
    f64::abs(x-y) < EPS
}

pub(crate) fn fneq(x: f64, y: f64) -> bool{
    !feq(x, y)
}

pub(crate) fn fgt(x: f64, y: f64) -> bool{
    if feq(x, y) {return false;}
    x > y
}

#[allow(dead_code)]
pub(crate) fn flt(x: f64, y: f64) -> bool{
    if feq(x, y) {return false;}
    x < y
}

pub(crate) fn fgeq(x: f64, y: f64) -> bool{
    if feq(x, y) {return true;}
    x > y
}

pub(crate) fn fleq(x: f64, y: f64) -> bool{
    if feq(x, y) {return true;}
    x < y
}