use std::fmt;
use crate::util::*;

/// This structure conceptullay represents a half-line (let us call it "Ray").
/// 
/// A ray has a "start vertex" **r<sub>0</sub>**, that is, **r<sub>0</sub>** is a part of the ray itself,
/// but we cannot make a disk of radius ε around **r<sub>0</sub>** which for every ε > 0.
/// 
/// If we consider the vectors from **r<sub>0</sub>** to each point on the ray, then they are all
/// pairwise parallel. Therefore there exists a "direction vector" **v** and we can
/// represent each point on ray by the parameterized equation:
/// 
/// <span style="display: inline-block; width: 100%; text-align:center;"> **r<sub>0</sub>** + *t***v** (*t* ≥ 0), </span>
/// 
/// where *t* is the parameter which is greater than or equal to zero.
/// 
/// You can also think of a ray as the locus of a moving point at a constant speed from the starting point **r<sub>0</sub>** as time passes.
/// In this case, the location of the point after time *t* (*t* ≥ 0) is equal to **r<sub>0</sub>** + *t***v**.
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
    /// Creates and returns a [Ray] w.r.t. the given arguments.
    ///  
    /// # Arguments
    ///  
    /// + `src`: The starting point of the ray.
    /// + `dst`: The point for which `src` is heading.
    /// 
    /// # Return
    /// 
    /// A `Ray` that starts from `src` towards `dst` with arrival time 1.
    /// 
    /// # Example
    /// 
    /// ```
    /// use polygon_offset::{Coordinate, Ray};
    /// 
    /// let c1 = (1., 2.).into();
    /// let c2 = (2., 3.).into();
    /// let r1 = Ray::new(c1, c2);
    /// 
    /// ```
    /// 
    pub fn new(src: Coordinate, dst: Coordinate) -> Self{
        Self {
            origin: src,
            angle: dst-src,
        }
    }

    /// Returns the "starting point" of the given ray.
    /// 
    /// # Example
    /// 
    /// ```
    /// use polygon_offset::{Coordinate, Ray};
    /// 
    /// let c1 = (1., 2.).into();
    /// let c2 = (2., 3.).into();
    /// let r1 = Ray::new(c1, c2);
    /// assert!(c1.eq(&r1.point()));
    /// 
    /// ```
    pub fn point(&self) -> Coordinate{
        self.point_by_ratio(0.)
    }

    /// Returns the value of parameterized equation **r<sub>0</sub>** + *t***v** by the given ratio *t*.
    /// 
    /// # Example
    /// 
    /// ```
    /// use polygon_offset::{Coordinate, Ray};
    /// 
    /// let c1 = (1., 2.).into();
    /// let c2 = (2., 3.).into();
    /// let c3 = (3., 4.).into();
    /// let r1 = Ray::new(c1, c2);
    /// 
    /// assert!(r1.point_by_ratio(2.).eq(&c3));
    /// ```
    pub fn point_by_ratio(&self, ratio: f64) -> Coordinate{
        self.origin + self.angle*ratio
    }

    pub(crate) fn bisector(&self, rhs: &Ray, origin: Coordinate, orient: bool) -> Self{
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

    /// Checks whether `self` contains the given Cartesian coordinate.
    /// Note that this function considers `self` as a open-ended line.
    /// That is, if the given point lies on the extended line of `self`, this function returns `true`.
    /// 
    /// # Return
    /// 
    /// + `true` if the given point lies on `self`,
    /// + `false` otherwise.
    /// 
    /// # Example
    /// 
    /// ```
    /// use polygon_offset::{Coordinate, Ray};
    /// 
    /// let c1 = (1., 2.).into();
    /// let c2 = (2., 3.).into();
    /// let c3 = (3., 4.).into();
    /// let r1 = Ray::new(c1, c2);
    /// 
    /// assert!(r1.is_contain(&c3));
    /// ```
    pub fn is_contain(&self, rhs: &Coordinate) -> bool {
        if self.is_degenerated() {return feq(self.origin.0, rhs.0) && feq(self.origin.1, rhs.1);}
        feq((*rhs - self.origin).outer_product(&self.angle), 0.)
    }

    /// Checks whether the given two rays are intersecting with each other.
    /// More precisely, it checks whether they have one or more common points.
    /// 
    /// # Return
    /// 
    /// + `true` if the given rays have one or more common points,
    /// + `false` otherwise.
    /// 
    /// # Example
    /// 
    /// ```
    /// use polygon_offset::{Coordinate, Ray};
    /// 
    /// let c1 = (1., 2.).into();
    /// let c2 = (2., 3.).into();
    /// let c3 = (3., 4.).into();
    /// let r1 = Ray::new(c1, c2);
    /// 
    /// assert!(r1.is_contain(&c3));
    /// ```
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