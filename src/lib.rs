//! The `geo-buffer` crate provides methods to offset (to inflate or deflate) certain 
//! primitive geometric types in the [GeoRust] ecosystem.
//! 
//! This crate can handle simple polygons properly as well as non-convex polygons, (valid) sets of polygons, and polygons with one or more holes.
//! Note that each method assumes **valid** primitives as a parameter, but [Polygon][Polygon module]/[MultiPolygon][MultiPolygon module] modules
//! *do not* enforce this validity automatically nor does this crate. (See more details on 'Validity' in [Polygon][Polygon module]/[MultiPolygon][MultiPolygon module]
//!  and [OGC standards].)
//! 
//! For now, the only viable geometric primitives are [Polygon][Polygon module] and [MultiPolygon][MultiPolygon module] (the rest of the primitives will be added as well).
//! 
//! # Quick Guide
//! 
//! The `buffer_polygon()` function (resp. `buffer_multi_polygon()` function) produces a `MultiPolygon` after applying
//! an offset operation to the given `Polygon` (resp. `MultiPolygon`). The absolute value of the argument passed with
//! determines the distance between each edge of the result multi-polygon and the original input. The sign determines the orientation
//! where the result expands. Positive values mean it going outward --- that is, it expands, --- and negative values mean going inward
//! --- it deflates ---.
//! 
//! Each code snippets below is a brief guide to use this crate. Click 'Result' to expand the visualized result.
//! (The red polygon designates the input, and the orange one designates the results.)
//! 
//! ### Example 1
//! 
//! You can manipulate a polygon with ease by a single function call.
//! 
//! ```
//! use geo_buffer::buffer_polygon;
//! use geo::{Polygon, MultiPolygon, LineString};
//! 
//! let p1 = Polygon::new(
//!     LineString::from(vec![(0., 0.), (1., 0.), (1., 1.), (0., 1.)]), vec![],
//! );
//! let p2: MultiPolygon = buffer_polygon(&p1, -0.2);
//! 
//! let expected_exterior = LineString::from(vec![(0.2, 0.2), (0.8, 0.2), (0.8, 0.8), (0.2, 0.8), (0.2, 0.2)]);
//! assert_eq!(&expected_exterior, p2.0[0].exterior())
//! 
//! ```
//! <details>
//! <summary style="cursor:pointer"> Result </summary>
//! <img src="./../../../ex1.svg" style="padding: 25px 30%;"/>
//! </details>
//! 
//! ### Example 2
//! 
//! This example shows the case where the polygon is split while it deflates.
//! 
//! ```
//! use geo_buffer::buffer_polygon;
//! use geo::{Polygon, MultiPolygon, LineString};
//! 
//! let p1 = Polygon::new(
//!     LineString::from(vec![(0., 0.), (4., 0.), (4., 4.), (2., 1.), (0., 4.)]), vec![],
//! );
//! let p2: MultiPolygon = buffer_polygon(&p1, -0.45);
//! 
//! ```
//! <details>
//! <summary style="cursor:pointer"> Result </summary>
//! <img src="./../../../ex2.svg" style="padding: 25px 30%;"/>
//! </details>
//! 
//! ### Example 3
//! 
//! You can apply this function to a set of `Polygon`s (i.e. `MultiPolygon`). The constituent polygons may be integrated while they expand.
//! 
//! ```
//! let p1 = Polygon::new(
//!     LineString::from(vec![(0., 0.), (2., 0.), (2., 2.), (0., 2.)]), vec![],
//! );
//! let p2 = Polygon::new(
//!     LineString::from(vec![(3., 3.), (5., 3.), (5., 5.), (3., 5.)]), vec![],
//! );
//! let mp1 = MultiPolygon::new(vec![p1, p2]);
//! let mp2 = geo_buffer::buffer_multi_polygon(&mp1, 0.9);
//! 
//! ```
//! <details>
//! <summary style="cursor:pointer"> Result </summary>
//! <img src="./../../../ex3.svg" style="padding: 25px 30%;"/>
//! </details>
//! 
//! ### Example 4
//! 
//! If you want to apply this function to each member (and not want to unify them), just traversing over an iterator and collecting them will be fine.
//! (You can get a vector of `MultiPolygon`s thanks to the 'turbofish' syntax:`::<>`.)
//! 
//! ```
//! let p1 = Polygon::new(
//!     LineString::from(vec![(0., 0.), (2., 0.), (2., 2.), (0., 2.)]), vec![],
//! );
//! let p2 = Polygon::new(
//!     LineString::from(vec![(3., 3.), (5., 3.), (5., 5.), (3., 5.)]), vec![],
//! );
//! let mp1 = MultiPolygon::new(vec![p1, p2]);
//! let mp2 = mp1.0.iter().map(|x| geo_buffer::buffer_polygon(x, 0.9)).collect::<Vec<_>>();
//! 
//! ```
//! <details>
//! <summary style="cursor:pointer"> Result </summary>
//! <img src="./../../../ex4.svg" style="padding: 25px 30%;"/>
//! </details>
//! 
//! # Reference
//! 
//! This is a Rust implementation of this paper[^note1][^note2]. (See also [Notes](#Notes) below.)
//! 
//! # Notes
//! 
//! It has been shown that the algorithm presented in this paper is incorrect.[^note3] Thus we slightly modified the algorithm for some edge cases.
//! 
//! 
//! [GeoRust]: https://georust.org
//! [Polygon module]: https://docs.rs/geo/0.24.1/geo/geometry/struct.Polygon.html
//! [MultiPolygon module]: https://docs.rs/geo/0.24.1/geo/geometry/struct.MultiPolygon.html
//! [OGC standards]: https://www.ogc.org/standard/sfa/
//! [^note1]: Felkel, Petr; Obdržálek, Štěpán (1998), *"Straight skeleton implementation"*, SCCG 98: Proceedings of the 14th Spring Conference on Computer Graphics, pp. 210–218.
//! 
//! [^note2]: The implementation of the straight skeleton algorithm in CGAL (The Computational Geometry Algorithms Library) is also based on this paper.
//! 
//! [^note3]: Huber, Stefan (2012), *Computing Straight Skeletons and Motorcycle Graphs: Theory and Practice*, Shaker Verlag.
//! 

// Define submodules and re-exports

mod priority_queue;
mod vertex_queue;
pub mod util;
pub mod skeleton;

pub use util::{Coordinate, Ray};

// Main functions in this module

use geo_types::{Polygon, MultiPolygon, LineString};
use skeleton::Skeleton;

/// This function returns the buffered (multi-)polygon of the given polygon.
/// 
/// # Arguments
/// 
/// + `input_polygon`: `Polygon` to buffer
/// + `distance`: determine how distant from each edge of original polygon to each edge of the result polygon
///     - `+` for to enlarge (to add paddings, make bigger) the given polygon
///     - `-` for to deflate (to add margins, make smaller) the given polygon
/// 
/// # Example
/// 
/// ```
/// use geo_buffer::buffer_polygon;
/// use geo::{Polygon, MultiPolygon, LineString};
///
/// let p1 = Polygon::new(
///     LineString::from(vec![(0., 0.), (1., 0.), (1., 1.), (0., 1.)]), vec![],
/// );
/// let p2: MultiPolygon = buffer_polygon(&p1, -0.2);
///
/// let expected_exterior = LineString::from(vec![(0.2, 0.2), (0.8, 0.2), (0.8, 0.8), (0.2, 0.8), (0.2, 0.2)]);
/// assert_eq!(&expected_exterior, p2.0[0].exterior())
///
/// ```
pub fn buffer_polygon(input_polygon: &Polygon, distance: f64) -> MultiPolygon{
    buffer_multi_polygon(&MultiPolygon::new(vec![input_polygon.clone()]), distance)
}

/// This function returns the buffered (multi-)polygon of the given multi-polygon.
/// 
/// # Arguments
/// 
/// + `input_multi_polygon`: `MultiPolygon` to buffer
/// + `distance`: determine how distant from each edge of original polygon to each edge of the result polygon
///     - `+` for to enlarge (to add paddings, make bigger) the given polygon
///     - `-` for to deflate (to add margins, make smaller) the given polygon
/// 
/// # Example
/// 
/// ```
/// use geo_buffer::buffer_polygon;
/// use geo::{Polygon, MultiPolygon, LineString};
///
/// let p1 = Polygon::new(
///     LineString::from(vec![(0., 0.), (2., 0.), (2., 2.), (0., 2.)]), vec![],
/// );
/// let p2 = Polygon::new(
///     LineString::from(vec![(3., 3.), (5., 3.), (5., 5.), (3., 5.)]), vec![],
/// );
/// let mp1 = MultiPolygon::new(vec![p1, p2]);
/// let mp2 = buffer_multi_polygon(&mp1, 1.);
/// let expected_exterior = LineString::from(vec![(-1., -1.), (3., -1.), (3., 2.), (6., 2.), (6., 6.), (2., 6.), (2., 3.), (-1., 3.), (-1., -1.)]);
/// assert_eq!(&expected_exterior, mp2.0[0].exterior())
///
/// ```
pub fn buffer_multi_polygon(input_multi_polygon: &MultiPolygon, distance: f64) -> MultiPolygon{
    let orientation = if distance < 0. {true} else {false};
    let offset_distance = f64::abs(distance);
    let skel = Skeleton::skeleton_of_polygon_vector(&input_multi_polygon.0, orientation);
    let vq = skel.get_vertex_queue(offset_distance);
    skel.apply_vertex_queue(&vq, offset_distance)
}

pub fn skeleton_of_polygon(input_polygon: &Polygon, orientation: bool) -> Skeleton{
    Skeleton::skeleton_of_polygon(input_polygon, orientation)
}

pub fn skeleton_of_multi_polygon(input_multi_polygon: &MultiPolygon, orientation: bool) -> Skeleton{
    Skeleton::skeleton_of_polygon_vector(&input_multi_polygon.0, orientation)
}

pub fn skeleton_of_polygon_to_linestring(input_polygon: &Polygon, orientation: bool) -> Vec<LineString>{
    Skeleton::skeleton_of_polygon(input_polygon, orientation).to_linestring()
}

pub fn skeleton_of_multi_polygon_to_linestring(input_multi_polygon: &MultiPolygon, orientation: bool) -> Vec<LineString>{
    Skeleton::skeleton_of_polygon_vector(&input_multi_polygon.0, orientation).to_linestring()
}

#[cfg(test)]
mod tests;

