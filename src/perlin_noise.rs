#![allow(dead_code)]
#![allow(unused_variables)]

// Perlin Noise implementation, derived from source: https://en.wikipedia.org/wiki/Perlin_noise#Implementation

fn interpolate(a0: f64, a1: f64, w: f64) -> f64 {
    (a1 - a0) * w + a0
}

struct Vector2 {
    x: f64,
    y: f64,
}

fn random_gradient(ix: i32, iy: i32) -> Vector2 {
    let random = 2920_f64
        * (f64::from(ix) * 21942_f64 + f64::from(iy) * 171324_f64 + 8912_f64).sin()
        * (f64::from(ix) * 23157_f64 * f64::from(iy) * 217832_f64 + 9758_f64).cos();
    Vector2 {
        x: random.cos(),
        y: random.sin(),
    }
}

fn dot_grid_gradient(ix: i32, iy: i32, x: f64, y: f64) -> f64 {
    let gradient = random_gradient(ix, iy);
    let dx = x - f64::from(ix);
    let dy = y - f64::from(iy);
    dx * gradient.x + dy * gradient.y
}

pub fn perlin(x: f64, y: f64) -> f64 {
    let x0 = x as i32;
    let x1 = x0 + 1;
    let y0 = y as i32;
    let y1 = y0 + 1;

    let sx = x - x0 as f64;
    let sy = y - y0 as f64;

    let (mut n0, mut n1, ix0, ix1, value): (f64, f64, f64, f64, f64);

    n0 = dot_grid_gradient(x0, y0, x, y);
    n1 = dot_grid_gradient(x1, y0, x, y);
    ix0 = interpolate(n0, n1, sx);

    n0 = dot_grid_gradient(x0, y1, x, y);
    n1 = dot_grid_gradient(x1, y1, x, y);
    ix1 = interpolate(n0, n1, sx);

    value = interpolate(ix0, ix1, sy);
    value
}
