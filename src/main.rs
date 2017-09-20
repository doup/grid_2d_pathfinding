#[macro_use]
extern crate glium;

mod renderer;
mod scene;

use glium::{glutin};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("Grid 2D Pathfinding").with_dimensions(800, 500);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

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

    scene.show_map(0);

    renderer.update_all(&display, &scene);

    let mut closed = false;
    while !closed {
        renderer.draw(&display);

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == glutin::ElementState::Pressed {
                            match input.virtual_keycode.unwrap() {
                                glutin::VirtualKeyCode::Escape => closed = true,
                                glutin::VirtualKeyCode::Key1 => scene.show_map(0),
                                glutin::VirtualKeyCode::Key2 => scene.show_map(1),
                                _ => (),
                            }

                            renderer.update_all(&display, &scene);
                        }
                    },
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
                                        scene.set_origin(map_x, map_y);
                                        renderer.update_origin(&display, &scene);
                                        renderer.update_path(&display, &scene);
                                    },
                                    glutin::MouseButton::Left => {
                                        scene.set_target(map_x, map_y);
                                        renderer.update_target(&display, &scene);
                                        renderer.update_path(&display, &scene);
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
