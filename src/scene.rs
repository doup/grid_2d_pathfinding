#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    position: [i8; 2],
}

impl Point {
    pub fn new(x: i8, y: i8) -> Point {
        Point { position: [x, y] }
    }

    pub fn get_x(&self) -> i8 {
        self.position[0]
    }

    pub fn get_y(&self) -> i8 {
        self.position[1]
    }
} 

#[derive(Debug)]
pub struct Map {
    pub width: i8,
    pub height: i8,
    pub tiles: Vec<u8>,
}

impl Map {
    pub fn new(width: i8, height: i8, tiles: Vec<u8>) -> Map {
        Map { width: width, height: height, tiles: tiles }
    }

    pub fn get_tile(&self, point: Point) -> u8 {
        self.tiles[(point.get_x() + point.get_y() * self.width) as usize]
    }

    pub fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = vec![];
        let (x, y) = (point.get_x(), point.get_y());

        // Top
        if y - 1 >= 0 && self.get_tile(Point::new(x, y - 1)) != 1 {
            neighbors.push(Point::new(x, y - 1));
        }

        // Left
        if x - 1 >= 0 && self.get_tile(Point::new(x - 1, y)) != 1 {
            neighbors.push(Point::new(x - 1, y));
        }

        // Bottom
        if y + 1 < self.height && self.get_tile(Point::new(x, y + 1)) != 1 {
            neighbors.push(Point::new(x, y + 1));
        }

        // Right
        if x + 1 < self.width && self.get_tile(Point::new(x + 1, y)) != 1 {
            neighbors.push(Point::new(x + 1, y));
        }

        neighbors
    }
} 

#[derive(Debug)]
pub struct Scene {
    origin: Point,
    target: Point,
    maps: Vec<Map>,
    map: usize,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            origin: Point { position: [0, 0] },
            target: Point { position: [0, 0] },
            maps:   vec![],
            map:    0,
        }
    }

    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
    }

    pub fn set_target(&mut self, target: Point) {
        self.target = target;
    }

    pub fn add_map(&mut self, map: Map) {
        self.maps.push(map);
    }

    pub fn show_map(&mut self, id: usize) {
        self.map = id;
    }

    pub fn map(&self) -> &Map {
        &self.maps[self.map]
    }

    pub fn get_path(&self) -> Vec<Point> {
        vec![]
    }
}
