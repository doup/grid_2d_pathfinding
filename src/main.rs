mod scene;

use std::collections::HashMap;

#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

fn shape_from_map(map: &scene::Map, padding: f32, type_filter: u8) -> Vec<Vertex> {
    let mut shape   = vec![];
    let cell_width  = 2.0 / map.width as f32;
    let cell_height = 2.0 / map.height as f32;

    for y in 0..map.height {
        for x in 0..map.width {
            let left   = -1.0 + x as f32 * cell_width + padding;
            let top    =  1.0 - y as f32 * cell_height - padding;
            let bottom = top - cell_height + (2.0 * padding);
            let right  = left + cell_width - (2.0 * padding);

            if map.tiles[(x + (y * map.width)) as usize] == type_filter {
                shape.push(Vertex { position: [left,  top,    0.0] });
                shape.push(Vertex { position: [left,  top,    0.0] });
                shape.push(Vertex { position: [right, top,    0.0] });
                shape.push(Vertex { position: [left,  bottom, 0.0] });
                shape.push(Vertex { position: [right, bottom, 0.0] });
                shape.push(Vertex { position: [right, bottom, 0.0] });
            }
        }
    }

    shape
}

fn shape_from_gridpoint(map: &scene::Map, point: &scene::Point, width: f32, z: f32) -> Vec<Vertex> {
    let cell_height = 2.0 / map.height as f32;
    let cell_width = 2.0 / map.width as f32;
    let x = -1.0 + (point.get_x() as f32 * cell_width) + cell_width / 2.0;
    let y =  1.0 - (point.get_y() as f32 * cell_height) - cell_height / 2.0;

    let shape = vec![
        Vertex { position: [x - width, y + width, z] },
        Vertex { position: [x + width, y + width, z] },
        Vertex { position: [x - width, y - width, z] },
        Vertex { position: [x + width, y - width, z] },
    ];

    shape
}
/*
fn get_path(path_origin: &scene::Point, path_target: &scene::Point, came_from: &HashMap<(u8, u8), (u8, u8)>) -> Vec<(u8, u8)> {
    let mut path = vec![];
    let origin = (path_origin.get_x(), path_origin.get_y());
    let mut target = (path_target.get_x(), path_target.get_y());

    path.push(target);

    loop {
        let from = came_from.get(&target).unwrap().clone();
        path.push(from);
        target = from;

        if from == origin {
            break
        }
    }

    path
}*/

fn shape_from_path(map: &scene::Map, path: Vec<(u8, u8)>, width: f32, z: f32) -> Vec<Vertex> {
    let mut shape   = vec![];
    let cell_width  = 2.0 / map.width as f32;
    let cell_height = 2.0 / map.height as f32;

    for point in path {
        let x = -1.0 + point.0 as f32 * cell_width + cell_width / 2.0; 
        let y =  1.0 - point.1 as f32 * cell_height - cell_height / 2.0;
        
        let left   = x - width;
        let top    = y + width;
        let bottom = y - width;
        let right  = x + width;

        shape.push(Vertex { position: [left,  top,    z] });
        shape.push(Vertex { position: [left,  top,    z] });
        shape.push(Vertex { position: [right, top,    z] });
        shape.push(Vertex { position: [left,  bottom, z] });
        shape.push(Vertex { position: [right, bottom, z] });
        shape.push(Vertex { position: [right, bottom, z] });
    }

    shape
}
/*
// Breadth first search
fn generate_came_from(map: &scene::Map, position: (i16, i16)) -> HashMap<(i16, i16), (i16, i16)> {
    let mut frontier = vec![];
    let mut came_from = HashMap::new();

    frontier.push(position);
    came_from.insert(position, position);

    while frontier.len() > 0 {
        let current = frontier.remove(0);
        for next in get_neighbors(&map, current) {
            if !came_from.contains_key(&next) {
                frontier.push(next);
                came_from.insert(next, current);
            }
        }
    }

    came_from
}*/

fn main() {
    let mut mouse_position: (f64, f64) = (0.0, 0.0);
    let mut scene = scene::Scene::new();

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

/*
    let mut path_origin = GridPoint { x: 0, y: 0 };
    let mut path_target = GridPoint { x: 0, y: 0 };
    let grid1 = Grid {
        width: 9,
        height: 7,
        grid: vec![
            0, 1, 0, 0, 0, 0, 1, 0, 0,
            0, 1, 0, 0, 1, 1, 0, 0, 0,
            0, 0, 1, 0, 0, 1, 0, 1, 0,
            0, 0, 0, 1, 0, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 1, 1, 1, 0, 1, 0, 1,
            0, 1, 0, 0, 0, 0, 0, 0, 0,
        ],
    };
*/
    //let mut came_from = generate_came_from(&grid1, (path_origin.x, path_origin.y));

    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("Grid 2D Pathfinding").with_dimensions(800, 500);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    implement_vertex!(Vertex, position);

    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        void main() {
            gl_Position = vec4(position, 1.0);
        }
    "#;

    let walkable_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.8, 0.94, 1.0, 1.0);
        }
    "#;

    let wall_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.07, 0.31, 0.41, 1.0);
        }
    "#;

    let path_origin_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0, 0.68, 0.48, 1.0);
        }
    "#;

    let path_target_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.65, 0.01, 0.25, 1.0);
        }
    "#;

    let path_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.33, 0.09, 0.18, 1.0);
        }
    "#;

    let walkable_obj_shape = shape_from_map(&scene.map(), 0.01, 0);
    let walkable_obj_vertex_buffer = glium::VertexBuffer::new(&display, &walkable_obj_shape).unwrap();
    let walkable_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let walkable_obj_program = glium::Program::from_source(&display, vertex_shader_src, walkable_fragment_shader_src, None).unwrap();

    let wall_obj_shape = shape_from_map(&scene.map(), 0.0, 1);
    let wall_obj_vertex_buffer = glium::VertexBuffer::new(&display, &wall_obj_shape).unwrap();
    let wall_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let wall_obj_program = glium::Program::from_source(&display, vertex_shader_src, wall_fragment_shader_src, None).unwrap();
/*
    let mut path_obj_shape = shape_from_path(&grid1, get_path(&path_origin, &path_target, &came_from), 0.015, 0.1);
    let mut path_obj_vertex_buffer = glium::VertexBuffer::new(&display, &path_obj_shape).unwrap();
    let mut path_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let mut path_obj_program = glium::Program::from_source(&display, vertex_shader_src, path_fragment_shader_src, None).unwrap();

    let mut path_origin_obj_shape = shape_from_gridpoint(&grid1, &path_origin, 0.05, 0.2);
    let mut path_origin_obj_vertex_buffer = glium::VertexBuffer::new(&display, &path_origin_obj_shape).unwrap();
    let mut path_origin_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let mut path_origin_obj_program = glium::Program::from_source(&display, vertex_shader_src, path_origin_fragment_shader_src, None).unwrap();

    let mut path_target_obj_shape = shape_from_gridpoint(&grid1, &path_target, 0.03, 0.3);
    let mut path_target_obj_vertex_buffer = glium::VertexBuffer::new(&display, &path_target_obj_shape).unwrap();
    let mut path_target_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let mut path_target_obj_program = glium::Program::from_source(&display, vertex_shader_src, path_target_fragment_shader_src, None).unwrap();
*/
    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        target.draw(
            &walkable_obj_vertex_buffer,
            &walkable_obj_indices,
            &walkable_obj_program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();

        target.draw(
            &wall_obj_vertex_buffer,
            &wall_obj_indices,
            &wall_obj_program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();
/*
        target.draw(
            &path_obj_vertex_buffer,
            &path_obj_indices,
            &path_obj_program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();

        target.draw(
            &path_origin_obj_vertex_buffer,
            &path_origin_obj_indices,
            &path_origin_obj_program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();

        target.draw(
            &path_target_obj_vertex_buffer,
            &path_target_obj_indices,
            &path_target_obj_program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();
*/
        target.finish().unwrap();

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
                                    glutin::MouseButton::Left => {
                                        scene.set_origin(scene::Point::new(map_x, map_y));
                                        /*
                                        path_origin_obj_shape = shape_from_gridpoint(&grid1, &path_origin, 0.05, 0.1);
                                        path_origin_obj_vertex_buffer = glium::VertexBuffer::new(&display, &path_origin_obj_shape).unwrap();
                                        path_origin_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
                                        path_origin_obj_program = glium::Program::from_source(&display, vertex_shader_src, path_origin_fragment_shader_src, None).unwrap();

                                        came_from = generate_came_from(&grid1, (path_origin.x, path_origin.y));

                                        path_obj_shape = shape_from_path(&grid1, get_path(&path_origin, &path_target, &came_from), 0.015, 0.1);
                                        path_obj_vertex_buffer = glium::VertexBuffer::new(&display, &path_obj_shape).unwrap();
                                        path_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
                                        path_obj_program = glium::Program::from_source(&display, vertex_shader_src, path_fragment_shader_src, None).unwrap();
                                        */
                                    },
                                    glutin::MouseButton::Right => {
                                        scene.set_target(scene::Point::new(map_x, map_y));
                                        /*
                                        path_target_obj_shape = shape_from_gridpoint(&grid1, &path_target, 0.03, 0.2);
                                        path_target_obj_vertex_buffer = glium::VertexBuffer::new(&display, &path_target_obj_shape).unwrap();
                                        path_target_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
                                        path_target_obj_program = glium::Program::from_source(&display, vertex_shader_src, path_target_fragment_shader_src, None).unwrap();

                                        path_obj_shape = shape_from_path(&grid1, get_path(&path_origin, &path_target, &came_from), 0.015, 0.1);
                                        path_obj_vertex_buffer = glium::VertexBuffer::new(&display, &path_obj_shape).unwrap();
                                        path_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
                                        path_obj_program = glium::Program::from_source(&display, vertex_shader_src, path_fragment_shader_src, None).unwrap();
                                        */
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
