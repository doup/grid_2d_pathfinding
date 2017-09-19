#[macro_use]
extern crate glium;

mod renderer;
mod scene;

use glium::{glutin};

fn main() {
    let mut renderer = renderer::Renderer::new();
    let mut scene = scene::Scene::new();
    let mut mouse_position: (f64, f64) = (0.0, 0.0);

    scene.add_map(scene::Map::new(9, 7, vec![
        0, 1, 0, 0, 0, 0, 1, 0, 0,
        0, 1, 0, 0, 1, 1, 0, 0, 0,
        0, 0, 1, 0, 0, 1, 0, 1, 0,
        0, 0, 0, 1, 0, 1, 1, 1, 0,
        0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 1, 1, 1, 0, 1, 0, 1,
        0, 1, 0, 0, 0, 0, 0, 0, 0,
    ]));

    scene.add_map(scene::Map::new(6, 7, vec![
        0, 0, 0, 0, 0, 0,
        0, 1, 1, 1, 1, 0,
        0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 1, 0,
        0, 1, 1, 1, 1, 0,
        0, 0, 0, 0, 0, 0,
    ]));

    scene.set_origin(scene::Point::new(0, 0));
    scene.set_target(scene::Point::new(0, 0));
    scene.show_map(0);

    renderer.update_all(&scene);

    let mut closed = false;
    while !closed {
        renderer.draw();

        let display = &mut renderer.display;
        let events_loop = &mut renderer.events_loop;

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    glutin::WindowEvent::MouseMoved { position, .. } => mouse_position = position,
                    glutin::WindowEvent::MouseInput { state, button, .. } => match state {
                        glutin::ElementState::Pressed => {
                            let window_size = display.gl_window().get_inner_size_pixels().unwrap();
                            let map_x = ((scene.map().width as f64 * mouse_position.0) / window_size.0 as f64) as i8;
                            let map_y = ((scene.map().height as f64 * mouse_position.1) / window_size.1 as f64) as i8;
                            let is_walkable = scene.map().get_tile(scene::Point::new(map_x, map_y)) == 0;

                            if is_walkable {
                                match button {
                                    glutin::MouseButton::Right => {
                                        scene.set_origin(scene::Point::new(map_x, map_y));
                                        //renderer.update_origin(&scene);
                                        //renderer.update_path(&scene);
                                    },
                                    glutin::MouseButton::Left => {
                                        scene.set_target(scene::Point::new(map_x, map_y));
                                        //renderer.update_target(&scene);
                                        //renderer.update_path(&scene);
                                    },
                                    _ => (),
                                }
                            }
                        },
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
