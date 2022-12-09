extern crate vecmath;

use std::iter::from_fn;

pub type Point = self::vecmath::Vector2<i64>;
pub type FPoint = self::vecmath::Vector2<f64>;
pub type Vec3 = self::vecmath::Vector3<i64>;
pub type FVec3 = self::vecmath::Vector3<f64>;
pub type Vec4 = self::vecmath::Vector4<i64>;
pub type FVec4 = self::vecmath::Vector4<f64>;
pub type Mat4 = self::vecmath::Matrix4<i64>;
pub type FMat4 = self::vecmath::Matrix4<f64>;
pub type Mat3 = self::vecmath::Matrix3<i64>;


pub use self::vecmath::vec2_add as point_add;
pub use self::vecmath::vec2_dot as point_dot;
pub use self::vecmath::vec2_neg as point_neg;
pub use self::vecmath::vec2_normalized as point_normalize;
pub use self::vecmath::vec2_scale as point_mul;
pub use self::vecmath::vec2_square_len as point_square_length;
pub use self::vecmath::vec2_sub as point_sub;
pub use self::vecmath::vec3_add as vec_add;
pub use self::vecmath::vec3_cross as vec_cross;
pub use self::vecmath::vec3_dot as vec_dot;
pub use self::vecmath::vec3_neg as vec_neg;

pub const NORTH: Point = [0, -1];
pub const UP: Point = NORTH;
pub const NORTH_EAST: Point = [1, -1];
pub const UP_RIGHT: Point = NORTH_EAST;
pub const EAST: Point = [1, 0];
pub const RIGHT: Point = EAST;
pub const SOUTH_EAST: Point = [1, 1];
pub const DOWN_RIGHT: Point = SOUTH_EAST;
pub const SOUTH: Point = [0, 1];
pub const DOWN: Point = SOUTH;
pub const SOUTH_WEST: Point = [-1, 1];
pub const DOWN_LEFT: Point = SOUTH_WEST;
pub const WEST: Point = [-1, 0];
pub const LEFT: Point = WEST;
pub const NORTH_WEST: Point = [-1, -1];
pub const UP_LEFT: Point = NORTH_WEST;

// Hex directions
// https://www.redblobgames.com/grids/hexagons/
pub const HEX_E: Vec3 = [1, -1, 0];
pub const HEX_W: Vec3 = [-1, 1, 0];
pub const HEX_SE: Vec3 = [0, -1, 1];
pub const HEX_SW: Vec3 = [-1, 0, 1];
pub const HEX_NW: Vec3 = [0, 1, -1];
pub const HEX_NE: Vec3 = [1, 0, -1];

pub const HEX_ALT_SE: Vec3 = [1, -1, 0];
pub const HEX_ALT_NW: Vec3 = [-1, 1, 0];
pub const HEX_ALT_S: Vec3 = [0, -1, 1];
pub const HEX_ALT_SW: Vec3 = [-1, 0, 1];
pub const HEX_ALT_N: Vec3 = [0, 1, -1];
pub const HEX_ALT_NE: Vec3 = [1, 0, -1];

pub const DIRECTIONS: [Point; 4] = [NORTH, EAST, SOUTH, WEST];
pub const DIRECTIONS_INCL_DIAGONALS: [Point; 8] = [
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];
pub const HEX_DIRECTIONS: [Vec3; 6] = [HEX_E, HEX_W, HEX_SW, HEX_SE, HEX_NW, HEX_NE];

pub fn neighbors(p: Point) -> impl Iterator<Item = Point> {
    let mut diter = DIRECTIONS.iter();
    from_fn(move || diter.next().map(|d| point_add(p, *d)))
}

pub fn neighbors_incl_diagonals(p: Point) -> impl Iterator<Item = Point> {
    let mut diter = DIRECTIONS_INCL_DIAGONALS.iter();
    from_fn(move || diter.next().map(|d| point_add(p, *d)))
}

pub fn hex_neighbors(p: Vec3) -> impl Iterator<Item = Vec3> {
    let mut diter = HEX_DIRECTIONS.iter();
    from_fn(move || diter.next().map(|d| vec_add(p, *d)))
}

pub fn point_signum(p: Point) -> Point {
    [p[0].signum(), p[1].signum()]
}

pub fn parse_grid_to<'a, I, J, T>(lines: I, f: fn(char) -> T) -> Vec<Vec<T>>
    where
        I: IntoIterator<Item = &'a J>,
        J: AsRef<str> + 'a,
{
    lines
        .into_iter()
        .map(|x| AsRef::as_ref(x).chars().map(f).collect())
        .collect()
}

pub struct GridIteratorHelper {
    extents: (Point, Point),
    curr: Option<Point>,
}

impl Iterator for GridIteratorHelper {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some([x, y]) = self.curr {
            let c = if x < self.extents.1[0] {
                Some([x + 1, y])
            } else if y < self.extents.1[1] {
                Some([self.extents.0[0], y + 1])
            } else {
                None
            };
            let curr = self.curr;
            self.curr = c;
            curr
        } else {
            None
        }
    }
}

pub trait Grid<T>
    where
        T: PartialEq + Copy,
{
    fn get_value(&self, pos: Point) -> Option<T>;
    fn set_value(&mut self, pos: Point, value: T);
    fn extents(&self) -> (Point, Point);
    fn points(&self) -> GridIteratorHelper {
        let extents = self.extents();
        GridIteratorHelper {
            extents: extents,
            curr: Some(extents.0)
        }
    }
    fn flip_horizontal(&mut self);
    fn flip_vertical(&mut self);
    fn transpose(&mut self);
    fn rotate_90_cw(&mut self) {
        self.transpose();
        self.flip_horizontal();
    }
    fn rotate_180_cw(&mut self) {
        self.flip_vertical();
        self.flip_horizontal();
    }
    fn rotate_270_cw(&mut self) {
        self.transpose();
        self.flip_vertical();
    }
}

impl<T> Grid<T> for Vec<Vec<T>>
    where
        T: Clone + Copy + Default + PartialEq,
{
    fn get_value(&self, pos: Point) -> Option<T> {
        let [x, y] = pos;
        if let Some(line) = self.get(y as usize) {
            if let Some(p) = line.get(x as usize) {
                return Some(*p)
            }
        }
        None
    }

    fn set_value(&mut self, pos: Point, value: T){
        let [x, y] = pos;
        if let Some(line) = self.get_mut(y as usize) {
            if let Some(p) = line.get_mut(x as usize) {
                *p = value
            }
        }
    }

    fn extents(&self) -> (Point, Point) {
        if !self.is_empty() && !self[0].is_empty() {
            return (
                [0,0],
                [(self[0].len()-1) as i64, (self.len()-1) as i64],
            );
        }
        ([0,0],[0,0])
    }

    fn flip_horizontal(&mut self) {
        let ([minx, miny], [maxx, maxy]) = self.extents();
        let mut new_vec = self.clone();
        for y in miny..=maxy {
            for x in minx..=maxx {
                let v = self[y as usize][x as usize];
                let new_x = maxx - (x - minx);
                new_vec[y as usize][new_x as usize] = v;
            }
        }
        *self = new_vec;
    }

    fn flip_vertical(&mut self) {
        let ([minx, miny], [maxx, maxy]) = self.extents();
        let mut new_vec = self.clone();
        for y in miny..=maxy {
            for x in minx..=maxx {
                let v = self[y as usize][x as usize];
                let new_y = maxy - (y - miny);
                new_vec[new_y as usize][x as usize] = v;
            }
        }
        *self = new_vec;
    }

    fn transpose(&mut self) {
        let ([min_x, min_y], [max_x, max_y]) = self.extents();
        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        // Make a vec with the transposed dimensions
        let mut new_vec = Vec::with_capacity(width);
        for _ in min_x..=max_x {
            let mut row = Vec::with_capacity(height);
            row.resize_with(height, Default::default);
            new_vec.push(row);
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let v = self[y as usize][x as usize];
                new_vec[x as usize][y as usize] = v;
            }
        }
        *self = new_vec;
    }
}
