use std::f32::consts::PI;

pub fn delta(start: f32, now: f32, duration: f32) -> f32 {
    ((now - start) / duration).clamp(0.0, 1.0)
}

pub fn lerp1i(a: i32, b: i32, t: f32) -> i32 {

    a + ((b as f32 - a as f32) * t) as i32

}

pub fn lerp2i(a: (i32, i32), b: (i32, i32), t: f32) -> (i32, i32) {

    (
        a.0 + ((b.0 as f32 - a.0 as f32) * t) as i32,
        a.1 + ((b.1 as f32 - a.1 as f32) * t) as i32
    )
}

pub fn lerp4i(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32), t: f32) -> (i32, i32, i32, i32) {
    (
        a.0 + ((b.0 as f32 - a.0 as f32) * t) as i32,
        a.1 + ((b.1 as f32 - a.1 as f32) * t) as i32,
        a.2 + ((b.2 as f32 - a.2 as f32) * t) as i32,
        a.3 + ((b.3 as f32 - a.3 as f32) * t) as i32
    )
}

pub fn ease_in_out_sine(t: f32) -> f32 {
    -(f32::cos(PI * t) - 1.0) / 2.0
}

pub fn ease_in_out_quart(t: f32) -> f32 {
    if t < 0.5 {
        8.0 * t * t * t
    } else {
        1.0 - f32::powi(-2.0 * t + 2.0, 4) / 2.0
    }
}

