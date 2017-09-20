use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn set_position(&mut self, x: i8, y: i8) {
        self.position[0] = x;
        self.position[1] = y;
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
        let mut point = point.clone();

        self.clamp_point(&mut point);
        self.tiles[(point.get_x() + point.get_y() * self.width) as usize]
    }

    pub fn clamp_point(&self, point: &mut Point) {
        let mut x = point.get_x();
        let mut y = point.get_y();

        x = if x < 0 { 0 } else { x };
        x = if x >= self.width { self.width - 1 } else { x };
        y = if y < 0 { 0 } else { y };
        y = if y >= self.height { self.height - 1 } else { y };

        point.set_position(x, y);
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
    pub origin: Point,
    pub target: Point,
    maps: Vec<Map>,
    map: usize,
    came_from: HashMap<Point, Point>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            origin:    Point { position: [0, 0] },
            target:    Point { position: [0, 0] },
            maps:      vec![],
            map:       0,
            came_from: HashMap::new(),
        }
    }

    pub fn add_map(&mut self, map: Map) {
        self.maps.push(map);
    }

    // Breadth first search
    fn calc_came_from(&mut self) {
        let mut frontier = vec![];

        // TODO: Se pueden quitar estos `clones()`? Esta bien? O es sintoma de algo… :-/
        frontier.push(self.origin.clone());
        self.came_from.clear();
        self.came_from.insert(self.origin.clone(), self.origin.clone());

        while frontier.len() > 0 {
            let current = frontier.remove(0);
            for next in self.map().get_neighbors(&current) {
                if !self.came_from.contains_key(&next) {
                    frontier.push(next.clone());
                    self.came_from.insert(next, current.clone());
                }
            }
        }
    } 

    pub fn get_path(&self) -> Vec<Point> {
        let mut path = vec![];
        let mut target = self.target.clone();

        path.push(target.clone());

        loop {
            let from = self.came_from.get(&target).unwrap().clone();
            path.push(from.clone());
            target = from;

            if target == self.origin {
                break
            }
        }

        path
    }

    pub fn map(&self) -> &Map {
        &self.maps[self.map]
    }

    pub fn set_origin(&mut self, x: i8, y: i8) {
        let mut point = Point::new(x, y);

        self.map().clamp_point(&mut point);
        self.origin = point;
        self.calc_came_from();
    }

    pub fn set_target(&mut self, x: i8, y: i8) {
        let mut point = Point::new(x, y);

        self.map().clamp_point(&mut point);
        self.target = point;
    }

    pub fn show_map(&mut self, id: usize) {
        self.map = id;

        self.set_origin(0, 0);
        self.set_target(0, 0);
    }
}
