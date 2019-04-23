mod attractor;
mod geometry;
use image::{ImageBuffer, Rgb};
use crate::attractor::*;
use crate::geometry::*;
//impl Triangle{
//    fn fill(&mut )
//
//}

fn linear_stepper_3(start: (f64, f64, f64), end: (f64, f64, f64), step: f64) -> impl Fn(f64) -> ((u32, u32, u32)) {
    /*
        precompute steps on line in rgb space
        return closure which will index into precomputed vector based on bounded [0, 1] input
    */
    //let mut vals = Vec::new();

    |_| (0, 0, 0)
}

fn rounder(places: u32) -> impl Fn(f64) -> f64 {
    let power = i32::pow(10, places) as f64;
    move |x| (x * power).round() / power
}

fn main() {
    let canvas_size = 1000 as u32;
    let a = 10.0;
    let b = 28.0;
    let c = 8.0 / 3.0;

    let mut x = 0.1;
    let mut y = 0.0;
    let mut z = 0.0;

    let attractor = LorentzAttractor{a, b, c, step: 0.01,
        x:0.1,
        y:0.0,
        z:0.0};
    let mut pts2 = vec![CartPoint{x, y, z}];
    let mut x_min = 0.0;
    let mut x_max = 0.0;
    let mut y_min = 0.0;
    let mut y_max = 0.0;
    let mut z_min = 0.0;
    let mut z_max = 0.0;
    attractor.take((canvas_size * canvas_size) as usize).for_each(|element| {
        pts2.push(element);
        match (element.z > z_max, element.z < z_min) {
            (true, false) => z_max = element.z,
            (false, true) => z_min = element.z,
            _ => {}
        };
        match (element.x > x_max, element.x < x_min) {
            (true, false) => x_max = element.x,
            (false, true) => x_min = element.x,
            _ => {}
        };
        match (element.y > y_max, element.y < y_min) {
            (true, false) => y_max = element.y,
            (false, true) => y_min = element.y,
            _ => {}
        };
    });
    let normalized_scaled:Vec<Rgb<u8>> = pts2.iter()
        .map(|e| e.normalize((x_min, x_max), (y_min, y_max), (z_min, z_max)))
        .map(|e| e * 255.0)
        .map(|e| Rgb([e.x as u8, 125 as u8, e.z as u8]))
        .collect();
    let mut image = ImageBuffer::new(canvas_size, canvas_size);
    //let mut index = 0;
    for i in 0..canvas_size{
        for j in 0..canvas_size{
            image.put_pixel(i as u32, j as u32, normalized_scaled[((i*canvas_size)+j) as usize]);
            //image.put_pixel(i as u32, j as u32, normalized_scaled[index]);
            //index = index + 1;
            //println!("{}, {}, {}, {}", i, j, index, (i*canvas_size)+j);

        }
    }
    image.save("chaos6_noGreen_1000.png");
//    let pts2:Vec<(f64,f64,f64)> = attractor
//        .take(1000).collect();
    println!("pts2 length {}, {}, {},{},{},{},{}", pts2.len(), x_min, x_max, y_min, y_max, z_min, z_max);
    println!("{},{},{},{},{},{}", x_lower, x_upper, y_lower, y_upper, z_lower, z_upper);
   // println!("normalized_scaled {}", normalized_scaled[0]);
}
