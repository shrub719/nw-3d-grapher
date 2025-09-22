const PI: f32 = 3.14159265;
const TWO_PI: f32 = 6.28318531;

fn wrap_pi(mut x: f32) -> f32 {
    while x < 0.0 { x += TWO_PI }
    while x >= TWO_PI { x -= TWO_PI }

    if x >= PI { x -= TWO_PI }

    x
}

pub fn sin(mut x: f32) -> f32 {
    x = wrap_pi(x);
    let x3 = x * x * x;
    let x5 = x3 * x * x;
    let x7 = x5 * x * x;
    x - x3 / 6.0 + x5 / 120.0 - x7 / 5040.0
}

pub fn cos(mut x: f32) -> f32 {
    x = wrap_pi(x);
    let x2 = x * x;
    let x4 = x2 * x2;
    let x6 = x4 * x2;
    return 1.0 - x2 / 2.0 + x4 / 24.0 - x6 / 720.0;
}
