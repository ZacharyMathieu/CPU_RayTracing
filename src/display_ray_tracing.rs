use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{
    frame::Frame, observer::Observer, parameters::RayParameters, polygon::Polygon, sphere::Sphere,
};

pub fn display(
    observer: &mut Observer,
    sphere_vector: &Vec<Sphere>,
    polygon_vector: &Vec<Polygon>,
    ray_parameters: &RayParameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(ray_parameters.background_color);
    canvas.clear();

    let mut observer_bodies: Vec<Sphere> = vec![];
    observer_bodies.push(observer.body);

    let frame: Frame = observer.get_next_frame(
        ray_parameters,
        &sphere_vector,
        &polygon_vector,
        &observer_bodies,
    );

    // Displaying the colors
    frame
        .colors
        .into_iter()
        .for_each(|((x, y), color): ((i64, i64), Color)| {
            canvas.set_draw_color(color);
            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
        });

    canvas.present();
}
