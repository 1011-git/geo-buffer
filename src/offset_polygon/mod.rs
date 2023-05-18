use geo_types::{Polygon, MultiPolygon, LineString};

use crate::skeleton::Skeleton;

pub fn offset_polygon(input_polygon: &Polygon, distance: f64) -> MultiPolygon{
    offset_multi_polygon(&MultiPolygon::new(vec![input_polygon.clone()]), distance)
}

pub fn offset_multi_polygon(input_polygon: &MultiPolygon, distance: f64) -> MultiPolygon{
    let orientation = if distance < 0. {true} else {false};
    let offset_distance = f64::abs(distance);
    let skel = Skeleton::skeleton_of_polygon_vector(&input_polygon.0, orientation);
    let vq = skel.get_vertex_queue(offset_distance);
    skel.apply_vertex_queue(&vq, offset_distance)
}

pub fn skeleton_of_polygon(input_polygon: &Polygon, orientation: bool) -> Skeleton{
    Skeleton::skeleton_of_polygon(input_polygon, orientation)
}

pub fn skeleton_of_multi_polygon(input_polygon: &MultiPolygon, orientation: bool) -> Skeleton{
    Skeleton::skeleton_of_polygon_vector(&input_polygon.0, orientation)
}

pub fn skel(input_polygon: &MultiPolygon, distance: f64) -> Vec<LineString>{
    let orientation = if distance < 0. {true} else {false};
    let skel = Skeleton::skeleton_of_polygon_vector(&input_polygon.0, orientation);
    skel.to_linestring()
}