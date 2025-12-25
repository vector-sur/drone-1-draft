mod distance;
mod interfaces;

use distance::get_distance;
use interfaces::gps as dron_gps;
use geo::prelude::*;
use geo::{Point, Haversine};

fn main() {
    //system_init();
    let from = dron_gps::get_position(); 
    let to = Point::new(-34.9042261, -57.9275761); // to is actually get from the web interface
    
    let distance = get_distance(from, to);
    let direction = Haversine.bearing(from, to);

    start_trip(from, to, distance, direction); // check everighti and start trip (up 100m)

    let actual_position = dron_gps::get_position();

    // trip to go
    while verify_distance(actual_position, to) > 1.0 {
        verify_height();
        verify_direction();
        move_forward();
        actual_position = dron_gps::get_position();
    }

    land();
    
    // trip to come back
    while verify_distance(actual_position, from) > 1.0 {
        verify_height();
        verify_direction();
        move_forward();
        actual_position = dron_gps::get_position();
    }

    land();
    end_trip();
}

