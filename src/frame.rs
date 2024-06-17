use std::collections::HashMap;

use sdl2::pixels::Color;

use crate::ray_trace::RayTrace;

struct PixelAccumulator {
    pub r: u64,
    pub g: u64,
    pub b: u64,
    pub a: u64,
    pub count: u64,
}

impl PixelAccumulator {
    pub fn get_color(&self) -> Color {
        return Color {
            r: (self.r / self.count) as u8,
            g: (self.g / self.count) as u8,
            b: (self.b / self.count) as u8,
            a: (self.a / self.count) as u8,
        };
    }
}

pub struct Frame {
    pub colors: HashMap<(i32, i32), Color>,
}

impl Frame {
    pub fn create_from_ray_trace(traces: Vec<RayTrace>) -> Frame {
        let mut colors: HashMap<(i32, i32), Color> = HashMap::new();

        traces.iter().for_each(|trace: &RayTrace| {
            colors.insert((trace.ray.x_value, trace.ray.y_value), trace.color);
        });

        return Frame { colors: colors };
    }

    fn create_empty() -> Frame {
        return Frame {
            colors: HashMap::new(),
        };
    }

    pub fn accumulate_frames(frames: &Vec<Frame>) -> Frame {
        let pixels: &mut HashMap<(i32, i32), PixelAccumulator> = &mut HashMap::new();

        frames.iter().for_each(|frame: &Frame| {
            frame
                .colors
                .iter()
                .for_each(|(position, color): (&(i32, i32), &Color)| {
                    if pixels.contains_key(&position) {
                        let accumulator: &mut PixelAccumulator = pixels.get_mut(position).unwrap();

                        accumulator.r += color.r as u64;
                        accumulator.g += color.g as u64;
                        accumulator.b += color.b as u64;
                        accumulator.a += color.a as u64;
                        accumulator.count += 1;
                    } else {
                        pixels.insert(
                            *position,
                            PixelAccumulator {
                                r: color.r as u64,
                                g: color.g as u64,
                                b: color.b as u64,
                                a: color.a as u64,
                                count: 1,
                            },
                        );
                    }
                });
        });

        let mut new_frame: Frame = Frame::create_empty();

        pixels.iter().for_each(
            |(position, accumulator): (&(i32, i32), &PixelAccumulator)| {
                new_frame.colors.insert(*position, accumulator.get_color());
            },
        );

        return new_frame;
    }
}
