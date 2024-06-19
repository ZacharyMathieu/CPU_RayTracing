use rand::{distributions::uniform::SampleBorrow, Rng};
use sdl2::pixels::Color;

pub fn rand_range<T>(rng: &mut rand::prelude::ThreadRng, low: T, high: T) -> T
where
    T: std::cmp::PartialEq + SampleBorrow<T> + rand::distributions::uniform::SampleUniform,
{
    return if low == high {
        low
    } else {
        rng.gen_range(low, high)
    };
}

pub fn float_to_color(f: f64) -> Color {
    return Color::RGB((f * 255.) as u8, 0, ((1. - f) * 255.) as u8);
}

pub fn rand_color(rng: &mut rand::prelude::ThreadRng) -> Color {
    return Color::RGB(
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
        rng.gen_range(0, 255),
    );
}

pub fn round(number: f64, decimals: u32) -> f64 {
    let mult: f64 = (10 as f64).powf(decimals as f64) as f64;
    return f64::round(number * mult) / mult;
}
