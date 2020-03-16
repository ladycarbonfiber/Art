use palette::{Gradient, Lch, LinSrgb, Srgb, IntoColor};
use image::{RgbImage, Rgb};

fn main() {
    println!("Hello, world!");
    let mut image = RgbImage::new(256,256);
    let color = LinSrgb::new(1.0 * 255., 0.0, 1.0*255.);

    for i in 0.. 256 {
        for j in 0.. 256 {
            image.put_pixel(i as u32, j as u32, hsv_to_rgb(i as f64, i as f64 / 255., j as f64 / 255.));
        }
    }
    image.save("test_hsv_grad.png").expect("saved successfully");
}
fn hsv_to_rgb(h:f64, s:f64, v:f64) -> Rgb<u8> {
    let c = v * s;
    let x = c * (1.-(h / 60. % 2. - 1.).abs());
    let m = v - c;

    let h_mod =  (h / 60.0).ceil() as i32;
    let (r_prime, g_prime, b_prime) = match h_mod {
        0 | 1 => {
            (c, x, 0.)
        },
        2 => {
            (x, c, 0.)
        },
        3 => {
            (0., c, x)
        },
        4 => {
            (0., x, c)
        }
        5 => {
            (x, 0., c)
        },
        6 => {
            (c, 0., x)
        }
        _ => {
            println!("invalid h {}", h);
            (0., 0., 0.)
        }
    };
    let r = ((r_prime +m ) * 255.) as u8;
    let g = ((g_prime +m) * 255.) as u8;
    let b = ((b_prime +m) * 255.) as u8;
    Rgb([r,g,b])
}
