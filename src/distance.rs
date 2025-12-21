use geo::prelude::*;
use geo::{Point, Haversine};

/// Calculate geodesic distance
pub fn get_distance(from: Point, to: Point) -> f64 {
    Haversine.distance(from, to)
}
