#[macro_use]
extern crate glium;

#[derive(Debug)]
struct Grid {
    width: u8,
    height: u8,
    grid: Vec<u8>,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

fn shape_from_grid(grid: &Grid, type_filter: u8) -> Vec<Vertex> {
    let mut shape   = vec![];
    let padding     = 0.01;
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

fn main() {
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

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.9, 0.9, 0.9, 1.0);
        }
    "#;

    let wall_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.7, 0.7, 0.7, 1.0);
        }
    "#;

    let walkable_obj_shape = shape_from_grid(&grid1, 0);
    let walkable_obj_vertex_buffer = glium::VertexBuffer::new(&display, &walkable_obj_shape).unwrap();
    let walkable_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let walkable_obj_program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let wall_obj_shape = shape_from_grid(&grid1, 1);
    let wall_obj_vertex_buffer = glium::VertexBuffer::new(&display, &wall_obj_shape).unwrap();
    let wall_obj_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let wall_obj_program = glium::Program::from_source(&display, vertex_shader_src, wall_fragment_shader_src, None).unwrap();

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
