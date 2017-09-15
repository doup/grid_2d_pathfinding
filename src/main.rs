#[macro_use]
extern crate glium;

#[derive(Debug)]
struct Grid {
    width: u8,
    height: u8,
    grid: Vec<u8>,
}

struct GridPoint {
    x: u8,
    y: u8,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

fn shape_from_grid(grid: &Grid, padding: f32, type_filter: u8) -> Vec<Vertex> {
    let mut shape   = vec![];
    let cell_width  = 2.0 / grid.width as f32;
    let cell_height = 2.0 / grid.height as f32;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let left   = -1.0 + x as f32 * cell_width + padding;
            let top    =  1.0 - y as f32 * cell_height - padding;
            let bottom = top - cell_height + (2.0 * padding);
            let right  = left + cell_width - (2.0 * padding);

            if grid.grid[(x + (y * grid.width)) as usize] == type_filter {
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

fn shape_from_gridpoint(grid: &Grid, point: &GridPoint, width: f32, z: f32) -> Vec<Vertex> {
    let cell_height = 2.0 / grid.height as f32;
    let cell_width = 2.0 / grid.width as f32;
    let x = -1.0 + (point.x as f32 * cell_width) + cell_width / 2.0;
    let y =  1.0 - (point.y as f32 * cell_height) - cell_height / 2.0;

    let shape = vec![
        Vertex { position: [x - width, y + width, z] },
        Vertex { position: [x + width, y + width, z] },
        Vertex { position: [x - width, y - width, z] },
        Vertex { position: [x + width, y - width, z] },
    ];

    shape
}

fn main() {
    let mut origin = GridPoint { x: 0, y: 0 };
    let mut target = GridPoint { x: 0, y: 0 };
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

    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
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

    let origin_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0, 0.68, 0.48, 1.0);
        }
    "#;

    let target_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.65, 0.01, 0.25, 1.0);
        }
    "#;

    let walkable_obj_shape = shape_from_grid(&grid1, 0.01, 0);
    let walkable_obj_vertex_buffer = glium::VertexBuffer::new(&display, &walkable_obj_shape).unwrap();
    let walkable_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let walkable_obj_program = glium::Program::from_source(&display, vertex_shader_src, walkable_fragment_shader_src, None).unwrap();

    let wall_obj_shape = shape_from_grid(&grid1, 0.0, 1);
    let wall_obj_vertex_buffer = glium::VertexBuffer::new(&display, &wall_obj_shape).unwrap();
    let wall_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let wall_obj_program = glium::Program::from_source(&display, vertex_shader_src, wall_fragment_shader_src, None).unwrap();

    let origin_obj_shape = shape_from_gridpoint(&grid1, &origin, 0.07, 0.1);
    let origin_obj_vertex_buffer = glium::VertexBuffer::new(&display, &origin_obj_shape).unwrap();
    let origin_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let origin_obj_program = glium::Program::from_source(&display, vertex_shader_src, origin_fragment_shader_src, None).unwrap();

    let target_obj_shape = shape_from_gridpoint(&grid1, &target, 0.03, 0.2);
    let target_obj_vertex_buffer = glium::VertexBuffer::new(&display, &target_obj_shape).unwrap();
    let target_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let target_obj_program = glium::Program::from_source(&display, vertex_shader_src, target_fragment_shader_src, None).unwrap();

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

        target.draw(
            &origin_obj_vertex_buffer,
            &origin_obj_indices,
            &origin_obj_program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();

        target.draw(
            &target_obj_vertex_buffer,
            &target_obj_indices,
            &target_obj_program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();

        target.finish().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });
    }
}
