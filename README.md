# geo-buffer
[![geo-buffer on crates.io](https://img.shields.io/crates/d/geo-buffer.svg)](https://crates.io/crates/geo-buffer)
[![Rust 1.68+](https://img.shields.io/badge/rust-1.68+-orange.svg)](https://www.rust-lang.org)
[![Cargo 1.68+](https://img.shields.io/badge/cargo-1.68+-blue.svg)](https://www.rust-lang.org)
[![Documentation](https://img.shields.io/docsrs/geo-buffer/latest.svg)](https://docs.rs/geo-buffer)

The `geo-buffer` crate provides methods to buffer (to enlarge or reduce) certain 
primitive geometric types in the [GeoRust] ecosystem via a straight skeleton.

This crate can handle simple polygons properly as well as non-convex polygons, (valid) sets of polygons, and polygons with one or more holes.
Note that each method assumes **valid** primitives as a parameter, but [Polygon][Polygon module]/[MultiPolygon][MultiPolygon module] modules
*do not* enforce this validity automatically nor does this crate. (See more details on 'Validity' in [Polygon][Polygon module]/[MultiPolygon][MultiPolygon module]
 and [OGC standards].)

This crate uses a [straight skeleton] to buffer (multi-)polygons. You can also compute a straight skeleton separately by proper methods.

For now, the only viable geometric primitives are [Polygon][Polygon module] and [MultiPolygon][MultiPolygon module] (the rest of the primitives will be added as well).

You can choose a style of convex corners with either options; mitter-joined or round-joined. (See more details on the document.)

## Quick Guide

The `buffer_polygon()` function (resp. `buffer_multi_polygon()` function) produces a `MultiPolygon` after applying
an buffer operation to the given `Polygon` (resp. `MultiPolygon`). The absolute value of the argument passed with
determines the distance between each edge of the result multi-polygon and the original input. The sign determines the direction
where the result expands. Positive values mean it goes outward &#8212 that is, it inflates, &#8212 and negative values mean goes inward
&#8212 it deflates &#8212.

Code snippets below is a brief guide to use this crate. Click 'Result' to expand the visualized result.
(The red polygon designates the input, and the orange one designates the results.)

### Example

```rust
use geo_buffer::buffer_polygon;
use geo::{Polygon, MultiPolygon, LineString};

let p1 = Polygon::new(
    LineString::from(vec![(0., 0.), (1., 0.), (1., 1.), (0., 1.)]), vec![],
);
let p2: MultiPolygon = buffer_polygon(&p1, -0.2);

let expected_exterior = LineString::from(vec![(0.2, 0.2), (0.8, 0.2), (0.8, 0.8), (0.2, 0.8), (0.2, 0.2)]);
assert_eq!(&expected_exterior, p2.0[0].exterior())

```
<details>
<summary style="cursor:pointer"> Result </summary>
<img src="./example.svg" style="padding: 25px 30%;"/>
</details>

---
## Usage
```toml
[dependencies]
geo-buffer = "0.1.0"
```

## Dependencies
```toml
[dependencies]
geo-types = "0.7.9"
geo = "0.24.1"
```

## Main Contributor
 * [Teneleven Corp](https://1011.co.kr)
 * [Inseop Youn](https://github.com/pinebananais)


## License
The geo-buffer crate is licensed under the Apache License (Version 2.0)
 * Details : [LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0)

## Contact
 * Teneleven Corp / support@1011.co.kr / https://1011.co.kr

[GeoRust]: https://georust.org
[Polygon module]: https://docs.rs/geo/0.24.1/geo/geometry/struct.Polygon.html
[MultiPolygon module]: https://docs.rs/geo/0.24.1/geo/geometry/struct.MultiPolygon.html
[OGC standards]: https://www.ogc.org/standard/sfa/
[straight skeleton]: https://en.wikipedia.org/wiki/Straight_skeleton