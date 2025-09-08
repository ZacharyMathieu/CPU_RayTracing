use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{frame::Frame, object::Object, observer::Observer, parameters::RayParameters};

pub fn display(
    observer: &mut Observer,
    object_vector: &Vec<Object>,
    ray_parameters: &RayParameters,
    canvas: &mut Canvas<Window>,
) {
    canvas.set_draw_color(ray_parameters.background_color);
    canvas.clear();

    let mut observer_bodies: Vec<Object> = vec![];
    observer_bodies.push(Object::Sphere(observer.body.clone()));

    let frame: Frame = observer.get_next_frame(
        ray_parameters,
        &object_vector.iter().chain(observer_bodies.iter()).collect(),
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
