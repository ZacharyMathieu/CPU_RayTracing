use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{frame::Frame, observer::Observer, parameters::RayParameters, sphere::Sphere};

pub fn display(
    observer: &mut Observer,
    sphere_vector: &Vec<Sphere>,
    ray_parameters: &RayParameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(ray_parameters.background_color);
    canvas.clear();

    let frame: Frame = observer.get_next_frame(ray_parameters, sphere_vector);

    // Displaying the colors
    frame
        .colors
        .into_iter()
        .for_each(|pixel: ((i32, i32), Color)| {
            canvas.set_draw_color(pixel.1);
            canvas
                .draw_point(Point::new(pixel.0 .0, pixel.0 .1))
                .unwrap();
        });

    canvas.present();
}
