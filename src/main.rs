#[macro_use]
extern crate glium;

mod renderer;
mod scene;

use glium::{glutin};

#[derive(Debug)]
struct MapPosition {
    pub x: i8,
    pub y: i8,
    pub is_walkable: bool,
}

fn get_map_position(viewport: &renderer::Rectangle, mouse_position: (f32, f32), map: &scene::Map, map_bbox: &renderer::Rectangle) -> MapPosition {
    let ogl_x = (mouse_position.0 / viewport.width) * 2.0 - 1.0;
    let ogl_y = -((mouse_position.1 / viewport.height) * 2.0 - 1.0);
    let mut map_x = (((ogl_x - map_bbox.x) * map.width as f32) / map_bbox.width) as i8;
    let mut map_y = -(((ogl_y - map_bbox.y) * map.height as f32) / map_bbox.height) as i8;

    map_x = if map_x < 0 { 0 } else if map_x > map.width - 1 { map.width - 1 } else { map_x };
    map_y = if map_y < 0 { 0 } else if map_y > map.height - 1 { map.height - 1 } else { map_y };

    MapPosition {
        x: map_x,
        y: map_y,
        is_walkable: map.get_tile(scene::Point::new(map_x, map_y)) == 0,
    }
}

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

    renderer.set_viewport(renderer::Rectangle { x: 0.0, y: 0.0, width: 800.0, height: 500.0 });
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
                            let map_position = get_map_position(
                                &renderer.get_viewport(),
                                (mouse_position.0 as f32, mouse_position.1 as f32),
                                scene.map(),
                                &renderer.get_map_bbox()
                            );

                            if map_position.is_walkable {
                                match button {
                                    glutin::MouseButton::Right => {
                                        scene.set_origin(map_position.x, map_position.y);
                                        renderer.update_origin(&display, &scene);
                                        renderer.update_path(&display, &scene);
                                    },
                                    glutin::MouseButton::Left => {
                                        scene.set_target(map_position.x, map_position.y);
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
