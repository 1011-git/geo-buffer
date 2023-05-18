//! The `polygon-offset` crate provides methods to offset (to enlarge or reduce) certain 
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
//! The `offset_polygon()` function (resp. `offset_multi_polygon()` function) produces a `MultiPolygon` after applying
//! an offset operation to the given `Polygon` (resp. `MultiPolygon`). The absolute value of the argument passed with
//! determines the distance between each edge of the result multi-polygon and the original input. The sign determines the orientation
//! where the result expands. Positive values mean it going outward --- that is, it expands, --- and negative values mean going inward
//! --- it shrinks ---.
//! 
//! The code snippets below are a brief guide to use this crate.
//! 
//! + You can manipulate a polygon with ease by a single function call.
//! 
//! ```
//! use polygon_offset::offset_polygon;
//! use geo::{Polygon, MultiPolygon, LineString};
//! 
//! let p1 = Polygon::new(
//!     LineString::from(vec![(0., 0.), (1., 0.), (1., 1.), (0., 1.)]),
//!     vec![],
//! );
//! let p2: MultiPolygon = offset_polygon(&p1, -0.2);
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
//! + This example shows the case where a polygon is split while it shrinks.
//! 
//! + You can apply this function to a set of `Polygon`s (i.e. `MultiPolygon`). The constituent polygons may be integrated while they expand.
//! 
//! + If you want to apply this function to each member (and not want to unify them), just traversing over iterator and collecting them will be fine.
//! 
//! 
//! # Reference
//! 
//! This is a Rust implementation of this paper[^note1][^note2]
//! 
//! [GeoRust]: https://georust.org
//! [Polygon module]: https://docs.rs/geo/0.24.1/geo/geometry/struct.Polygon.html
//! [MultiPolygon module]: https://docs.rs/geo/0.24.1/geo/geometry/struct.MultiPolygon.html
//! [OGC standards]: https://www.ogc.org/standard/sfa/
//! [^note1]: Felkel, Petr; Obdržálek, Štěpán (1998), "Straight skeleton implementation", SCCG 98: Proceedings of the 14th Spring Conference on Computer Graphics, pp. 210–218.
//! 
//! [^note2]: The implementation of the straight skeleton algorithm in CGAL (The Computational Geometry Algorithms Library) also basedon this paper.
//! 

pub mod util;
mod priority_queue;
mod vertex_queue;
mod skeleton;
mod offset_polygon;

pub use crate::offset_polygon::{offset_polygon, offset_multi_polygon, skel};

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}
