pub mod rhai_script_host;
pub mod script_stuff;

use coarsetime::{Duration, Instant};

/**
 * calculates the area of the x and y by multiplying the width and height
 */
pub fn area_of(x: i32, y: i32) -> i32 {
    x*y
}

/**
 * calculates the area of x, y and z by multiplying the width, height and depth
 */
pub fn volume(x: i32, y: i32, z: i32) -> i32 {
    x*y*z
}

pub fn blank_ln(){
    println!()
}

/**
 * do speed test then return the time
 */
pub fn speed_test() -> Duration{
    let start = Instant::now();
    let mut thing = 1;
    for i in 0..1000000000 {
        thing = thing * i;
    }
    println!("{}", thing);

    start.elapsed()
}

/**
 * print differences
 */
pub fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

/**
 * print coords
 */
pub fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

/**
 * DING
 */
pub fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

pub fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

/**
 * print distance to origen
 */
pub fn print_distance(x: f32,y: f32) {
    println!(
        "Distance to the origin is {}",
        ( x.powf(2.0) + y.powf(2.0) ).sqrt());
}

