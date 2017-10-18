use std::collections::HashMap;
use glium;
use glium::{Surface};
use scene;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

struct Object {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices:       glium::index::PrimitiveType,
    program:       glium::Program,
}

#[derive(Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
}

pub struct Renderer {
    viewport:        Rectangle,
    viewport_margin: u8,
    top_bar_height:  u8,
    walkable_obj:    Option<Object>,
    walls_obj:       Option<Object>,
    path_obj:        Option<Object>,
    path_origin_obj: Option<Object>,
    path_target_obj: Option<Object>,
    shaders:         HashMap<String, String>,
    map_bbox:        Rectangle,
}

impl Renderer {
    pub fn new() -> Renderer {
        let mut shaders: HashMap<String, String> = HashMap::new();
        
        shaders.insert(String::from("vertex.vert"), String::from(include_str!("shaders/vertex.vert")));
        shaders.insert(String::from("walkable.frag"), String::from(include_str!("shaders/walkable.frag")));
        shaders.insert(String::from("walls.frag"), String::from(include_str!("shaders/walls.frag")));
        shaders.insert(String::from("path.frag"), String::from(include_str!("shaders/path.frag")));
        shaders.insert(String::from("origin.frag"), String::from(include_str!("shaders/origin.frag")));
        shaders.insert(String::from("target.frag"), String::from(include_str!("shaders/target.frag")));

        implement_vertex!(Vertex, position);

        Renderer {
            viewport:        Rectangle { x: 0.0, y: 0.0, width: 1.0, height: 1.0 },
            viewport_margin: 30,
            top_bar_height:  30,
            walkable_obj:    None,
            walls_obj:       None,
            path_obj:        None,
            path_origin_obj: None,
            path_target_obj: None,
            shaders:         shaders,
            map_bbox:        Rectangle { x: -0.5, y: 0.5, width: 1.0, height: 1.0 },
        }
    }

    pub fn draw(&self, display: &glium::Display) {
        let mut target = display.draw();

        target.clear_color(1.0, 1.0, 1.0, 1.0);

        Renderer::draw_object(&mut target, &self.walkable_obj);
        Renderer::draw_object(&mut target, &self.walls_obj);
        Renderer::draw_object(&mut target, &self.path_obj);
        Renderer::draw_object(&mut target, &self.path_origin_obj);
        Renderer::draw_object(&mut target, &self.path_target_obj);

        target.finish().unwrap();
    }

    fn draw_object(target: &mut glium::Frame, object_data: &Option<Object>) {
        match *object_data {
            Some(ref object) => {
                target.draw(
                    &object.vertex_buffer,
                    glium::index::NoIndices(object.indices),
                    &object.program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default()
                ).unwrap();   
            },
            None => (),
        }
    }

    pub fn get_map_bbox(&self) -> &Rectangle {
        &self.map_bbox
    }

    pub fn get_viewport(&self) -> &Rectangle {
        &self.viewport
    }

    pub fn set_viewport(&mut self, viewport: Rectangle) {
        self.viewport = viewport;
    }

    pub fn update_all(&mut self, display: &glium::Display, scene: &scene::Scene) {
        self.update_walkable(display, scene);
        self.update_walls(display, scene);
        self.update_path(display, scene);
        self.update_origin(display, scene);
        self.update_target(display, scene);
    }

    pub fn update_walkable(&mut self, display: &glium::Display, scene: &scene::Scene) {
        let vertex_shader = self.shaders.get(&String::from("vertex.vert")).unwrap();
        let fragment_shader = self.shaders.get(&String::from("walkable.frag")).unwrap();

        self.walkable_obj = Some(Object {
            vertex_buffer: glium::VertexBuffer::new(display, &self.shape_from_map(&scene.map(), 0.01, 0)).unwrap(),
            indices: glium::index::PrimitiveType::TriangleStrip,
            program: glium::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap(),
        });
    }

    pub fn update_walls(&mut self, display: &glium::Display, scene: &scene::Scene) {
        let vertex_shader = self.shaders.get(&String::from("vertex.vert")).unwrap();
        let fragment_shader = self.shaders.get(&String::from("walls.frag")).unwrap();

        self.walls_obj = Some(Object {
            vertex_buffer: glium::VertexBuffer::new(display, &self.shape_from_map(&scene.map(), 0.0, 1)).unwrap(),
            indices: glium::index::PrimitiveType::TriangleStrip,
            program: glium::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap(),
        });
    }

    pub fn update_path(&mut self, display: &glium::Display, scene: &scene::Scene) {
        let vertex_shader_src = self.shaders.get(&String::from("vertex.vert")).unwrap();
        let fragment_shader = self.shaders.get(&String::from("path.frag")).unwrap();

        self.path_obj = Some(Object {
            vertex_buffer: glium::VertexBuffer::new(display, &self.shape_from_path(&scene.map(), scene.get_path(), 0.015, 0.1)).unwrap(),
            indices: glium::index::PrimitiveType::TriangleStrip,
            program: glium::Program::from_source(display, vertex_shader_src, fragment_shader, None).unwrap(),
        });
    }

    pub fn update_origin(&mut self, display: &glium::Display, scene: &scene::Scene) {
        let vertex_shader_src = self.shaders.get(&String::from("vertex.vert")).unwrap();
        let fragment_shader = self.shaders.get(&String::from("origin.frag")).unwrap();

        self.path_origin_obj = Some(Object {
            vertex_buffer: glium::VertexBuffer::new(display, &self.shape_from_point(&scene.map(), &scene.origin, 0.05, 0.2)).unwrap(),
            indices: glium::index::PrimitiveType::TriangleStrip,
            program: glium::Program::from_source(display, vertex_shader_src, fragment_shader, None).unwrap(),
        });
    }

    pub fn update_target(&mut self, display: &glium::Display, scene: &scene::Scene) {
        let vertex_shader_src = self.shaders.get(&String::from("vertex.vert")).unwrap();
        let fragment_shader = self.shaders.get(&String::from("target.frag")).unwrap();

        self.path_target_obj = Some(Object {
            vertex_buffer: glium::VertexBuffer::new(display, &self.shape_from_point(&scene.map(), &scene.target, 0.03, 0.3)).unwrap(),
            indices: glium::index::PrimitiveType::TriangleStrip,
            program: glium::Program::from_source(display, vertex_shader_src, fragment_shader, None).unwrap(),
        });
    }

    fn shape_from_map(&self, map: &scene::Map, padding: f32, type_filter: u8) -> Vec<Vertex> {
        let mut shape   = vec![];
        let cell_width  = self.map_bbox.width / map.width as f32;
        let cell_height = self.map_bbox.height / map.height as f32;

        for y in 0..map.height {
            for x in 0..map.width {
                let left   = self.map_bbox.x + x as f32 * cell_width + padding;
                let top    = self.map_bbox.y - y as f32 * cell_height - padding;
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

    fn shape_from_path(&self, map: &scene::Map, path: Vec<scene::Point>, width: f32, z: f32) -> Vec<Vertex> {
        let mut shape   = vec![];
        let cell_width  = self.map_bbox.width / map.width as f32;
        let cell_height = self.map_bbox.height / map.height as f32;

        for point in path {
            let x = self.map_bbox.x + point.get_x() as f32 * cell_width + cell_width / 2.0; 
            let y = self.map_bbox.y - point.get_y() as f32 * cell_height - cell_height / 2.0;
            
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

    fn shape_from_point(&self, map: &scene::Map, point: &scene::Point, width: f32, z: f32) -> Vec<Vertex> {
        let cell_height = self.map_bbox.width / map.height as f32;
        let cell_width = self.map_bbox.height / map.width as f32;
        let x = self.map_bbox.x + (point.get_x() as f32 * cell_width) + cell_width / 2.0;
        let y = self.map_bbox.y - (point.get_y() as f32 * cell_height) - cell_height / 2.0;

        let shape = vec![
            Vertex { position: [x - width, y + width, z] },
            Vertex { position: [x + width, y + width, z] },
            Vertex { position: [x - width, y - width, z] },
            Vertex { position: [x + width, y - width, z] },
        ];

        shape
    }
}
