use std::fmt;
use crate::util::*;

/// aasdf
#[derive(Clone, Default, Debug, Copy)]
pub struct Ray{
    pub(crate) origin: Coordinate,
    pub(crate) angle: Coordinate,
}

impl fmt::Display for Ray{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Origin : (x, y) = ({}, {}) / Angle : (dx, dy) = ({}, {})", self.origin.0, self.origin.1, self.angle.0, self.angle.1)
    }
}

impl Ray{
    /**
     * Returns a [Ray] w.r.t. the given arguments
     * 
     * # Arguments
     * 
     * + `src` - a coordinate
     * 
     */
    pub fn new(src: Coordinate, dst: Coordinate) -> Self{
        Self {
            origin: src,
            angle: dst-src,
        }
    }

    pub fn point(&self) -> Coordinate{
        self.point_by_ratio(0.)
    }

    pub fn point_by_ratio(&self, ratio: f64) -> Coordinate{
        self.origin + self.angle*ratio
    }

    pub fn bisector(&self, rhs: &Ray, origin: Coordinate, orient: bool) -> Self{
        let mut ray = self.angle*rhs.angle.norm() + rhs.angle*self.angle.norm();
        if feq(ray.0, 0.) && feq(ray.1, 0.) {
            ray = (-self.angle.1, self.angle.0).into();
            if orient {ray = ray * -1.;}
        }
        else  {
            if orient == true && self.angle.outer_product(&ray) > 0.0 {ray = ray*-1.0;}
            if orient == false && self.angle.outer_product(&ray) < 0.0 {ray = ray*-1.0;}
        }
        // else {
        //     if orient == true && tmp_angle.outer_product(&ray) > 0.0 {ray = ray*-1.0;}
        //     if orient == false && tmp_angle.outer_product(&ray) < 0.0 {ray = ray*-1.0;}
        // }
        Self { origin: origin, angle: ray }
    }

    pub fn is_contain(&self, rhs: &Coordinate) -> bool {
        if self.is_degenerated() {return feq(self.origin.0, rhs.0) && feq(self.origin.1, rhs.1);}
        feq((*rhs - self.origin).outer_product(&self.angle), 0.)
    }

    pub fn intersect(&self, rhs: &Ray) -> Coordinate{
        let op = self.angle.outer_product(&rhs.angle);
        if feq(op, 0.) {
            if self.is_contain(&rhs.origin) {
                if fgt((rhs.origin - self.origin)/self.angle, 0.) {return rhs.origin;}
                else {return self.origin;}
            }
            return (self.origin + rhs.origin)/2.0;
        }
        let i = (rhs.origin - self.origin).outer_product(&rhs.angle) / self.angle.outer_product(&rhs.angle);
        self.origin + self.angle*i
    }

    pub fn is_intersect(&self, rhs: &Ray) -> bool {
        let op = self.angle.outer_product(&rhs.angle);
        if feq(op, 0.0){
            if self.is_contain(&rhs.origin) {return true;}
            if rhs.is_contain(&self.origin) {return true;}
            return false;
        }
        let i = (rhs.origin - self.origin).outer_product(&rhs.angle) / self.angle.outer_product(&rhs.angle);
        let j = (rhs.origin - self.origin).outer_product(&self.angle) / self.angle.outer_product(&rhs.angle);
        if fgeq(i, 0.) && fgeq(j, 0.) {return true;}
        false
    }

    pub fn is_parallel(&self, rhs: &Ray) -> bool {
        let op = self.angle.outer_product(&rhs.angle);
        if feq(op, 0.0) && !self.is_contain(&rhs.origin) {return true;}
        return false;
    }
    pub fn is_degenerated(&self) -> bool {
        if feq(self.angle.0, 0.) && feq(self.angle.1, 0.) {true} else {false}
    }

    pub fn normalize(&mut self) {
        if self.is_degenerated() {return;}
        self.angle = self.angle/self.angle.norm();
    }

    pub fn orientation(&self, rhs: &Coordinate) -> i32 {
        let res = self.angle.outer_product(&(*rhs - self.origin));
        if feq(res, 0.) {return 0;}
        if fgt(res, 0.) {return 1;}
        return -1;
    }

    pub fn reverse(&self) -> Self{
        Self{
            origin: self.origin,
            angle: self.angle*-1.,
        }
    }
}