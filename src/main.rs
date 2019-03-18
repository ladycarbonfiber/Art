use image::{ImageBuffer, Rgb};

fn lorentz_attractor(a: f64, b: f64, c: f64, step: f64) -> impl Fn(f64, f64, f64) -> (f64, f64, f64) {
    //dx/dt = a(y-x)
    //dy/dt = x(b-z) -y
    //dz/dt = xy -cz
    move |x, y, z| (x + (step * a * (y - x)), y + (step * (x * (b - z) - y)), z + (step * (x * y - c * z)))
}
fn linear_stepper_3(start:(f64, f64, f64), end:(f64, f64, f64), step:f64) -> impl Fn(f64) -> ((u32, u32, u32)){
    /*
        precompute steps on line in rgb space
        return closure which will index into precomputed vector based on bounded [0, 1] input
    */
    //let mut vals = Vec::new();

    |_| (0,0,0)

}

fn main() {
    let a = 10.0;
    let b = 28.0;
    let c = 8.0 / 3.0;

    let mut x = 0.1;
    let mut y = 0.0;

    //coordinates for bounding box
    let mut x_upper = 0.0;
    let mut x_lower = 0.0;

    let mut y_upper = 0.0;
    let mut y_lower = 0.0;
    //normalization for color space
    let mut z_upper = 0.0;
    let mut z_lower = 0.0;

    let mut z = 0.0;

    let mut pts = vec![(x, y, z)];
    //generate
    let l = lorentz_attractor(a, b, c, 0.01);
    for _ in 0..500 {
        let temp = l(x, y, z);
        x = temp.0;
        y = temp.1;
        z = temp.2;

        if z > z_upper{
            z_upper = z;
        }
        if z < z_lower{
            z_lower = z;
        }
        if x > x_upper{
            x_upper = x;
        }
        if x < x_lower {
            x_lower = x;
        }
        if y > y_upper{
            y_upper = y;
        }
        if y < y_lower{
            y_lower = y;
        }

        pts.push((x, y, z));
    }
    println!("{},{},{},{},{},{}", x_lower, x_upper, y_lower, y_upper, z_lower, z_upper);
    for entry in pts {
        println!("{:?}", entry);
    }
}
