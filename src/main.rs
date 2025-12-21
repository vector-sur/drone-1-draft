mod distance;
mod interfaces;

use distance::get_distance;
use interfaces::gps as dron_gps;
use geo::prelude::*;
use geo::{Point, Haversine};

fn main() {
    let from = dron_gps::get_position(); 
    let to = Point::new(-34.9042261, -57.9275761);
    
    let distance = get_distance(from, to);
    let direction = Haversine.bearing(from, to);

    println!("=== POSICIÓN ===");
    println!("From: {:#?}", from);
    println!("To: {:#?}", to);
    
    println!("\n=== DISTANCIA ===");
    println!("One way distance: {distance} [m]");
    println!("One way distance: {} [km]", distance / 1000.0);
    println!("Round trip distance: {} [m]", distance * 2.0);
    println!("Round trip distance: {} [km]", distance * 2.0 / 1000.0);
    
    println!("\n=== DIRECCIÓN ===");
    println!("Vector de dirección: {:#?}°", direction);
}