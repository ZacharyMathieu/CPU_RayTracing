use rand::Rng;
use sdl2::pixels::Color;

pub fn rand_range(rng: &mut rand::prelude::ThreadRng, low: f64, high: f64) -> f64 {
    return if low == high {
        low
    } else {
        rng.gen_range(low, high)
    };
}

// fn float_to_color(f: f64) -> Color {
//     return Color::RGB((f * 255.0) as u8, 0, ((1.0 - f) * 255.0) as u8);
// }

pub fn rand_color(rng: &mut rand::prelude::ThreadRng) -> Color {
    return Color::RGB(
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
    );
}
