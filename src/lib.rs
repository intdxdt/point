use serde::{Serialize, Deserialize};
use coordinate::Coordinate;
use std::fmt::Debug;
use math_util::{num, Feq, EPSILON, PI, TAU, NumCast};
use std::ops::{Index, IndexMut};
use robust_orientation::orientation_2d;


/// Point is a 2D (x:float, y:float) point type.
/// float : f32 & f64 - required for most computations
/// requiring area, distance, trigonometry, etc.
#[derive(Serialize, Deserialize, Copy, Clone, PartialOrd, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn new_from_array(a: &[f64; 2]) -> Point {
        Point { x: a[0], y: a[1] }
    }

    ///Operator : equals
    pub fn equals(&self, other: &Point) -> bool {
        self.x.feq(other.x) && self.y.feq(other.y)
    }

    ///As array
    #[inline]
    pub fn as_array(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    ///As tuple
    pub fn as_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    ///Computes vector magnitude given x an dy component
    pub fn magnitude(&self) -> f64 {
        self.x.hypot(self.y)
    }
    ///Computes vector square magnitude given pt as x, y components
    pub fn square_magnitude(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y)
    }

    ///Computes distance between two points
    pub fn distance(&self, other: &Self) -> f64 {
        self.sub(other).magnitude()
    }

    ///Point from magnitude and direction
    pub fn component(m: f64, d: f64) -> Point {
        Point::new(m * d.cos(), m * d.sin())
    }

    ///Dot Product of two points as vectors
    pub fn dot_product(&self, o: Point) -> f64 {
        (self.x * o.x) + (self.y * o.y)
    }

    ///2D cross product of AB and AC vectors,
    ///i.e. z-component of their 3D cross product.
    ///negative cw and positive if ccw
    pub fn cross_product(&self, b: Point) -> f64 {
        return (self.x * b.y) - (self.y * b.x);
    }

    ///Deflect_vector computes vector deflection given deflection angle and
    /// side of vector to deflect from (from_end)
    pub fn deflect(&self, mag: f64, defl_angle: f64, from_end: bool) -> Point {
        return self.extend(mag, PI - defl_angle, from_end);
    }

    ///kproduct scales x and y components by constant  k
    pub fn kproduct(&self, k: f64) -> Point {
        self.mult(k)
    }

    ///negative of point
    pub fn neg(&self) -> Point {
        self.kproduct(-1.0)
    }

    ///Dir computes direction in radians - counter clockwise from x-axis.
    pub fn direction(&self) -> f64 {
        let mut d = self.y.atan2(self.x);
        if d < 0.0 {
            d += TAU
        }
        return d;
    }

    ///Revdir computes the reversed direction from a foward direction
    pub fn reverse_direction(d: f64) -> f64 {
        let mut r = d - PI;
        if d < PI {
            r = d + PI;
        }
        return r;
    }

    ///deflection angle
    pub fn deflection_angle(bearing1: f64, bearing2: f64) -> f64 {
        let mut a = bearing2 - Point::reverse_direction(bearing1);
        if a < 0.0 {
            a += TAU;
        }
        return PI - a;
    }


    ///Unit vector of point
    pub fn unit_vector(&self) -> Point {
        let mut m = self.magnitude();
        if m.feq(0.0) {
            m = EPSILON;
        }
        Point::new(self.x / m, self.y / m)
    }

    ///Projects self on to v
    pub fn project(&self, v: Point) -> f64 {
        return self.dot_product(v.unit_vector());
    }

    ///2D cross product of AB and AC vectors given A, B, and C as points,
    ///i.e. z-component of their 3D cross product.
    ///Returns a positive value, if ABC makes a counter-clockwise turn,
    ///negative for clockwise turn, and zero if the points are collinear.
    pub fn orientation2d(&self, a: Point, b: Point) -> f64 {
        return orientation_2d(&a.as_array(), &b.as_array(), &self.as_array());
    }

    ///Extends vector from the end or beginning based on `from_end`.
    pub fn extend(&self, magnitude: f64, angle: f64, from_end: bool) -> Point {
        //from a of v back direction initiates as fwd v direction anticlockwise
        //bb - back bearing
        //fb - forward bearing
        let mut bb = self.direction();
        if from_end {
            bb += PI;
        }
        let mut fb = bb + angle;
        if fb > TAU {
            fb -= TAU;
        }
        return Point::component(magnitude, fb);
    }

    ///Distance from self to segment
    pub fn distance_to_segment(&self, sa: Point, sb: Point) -> f64 {
        self.distance_to_seg(sa, sb, f64::hypot)
    }

    ///Square Distance from self to segment
    pub fn square_distance_to_segment(&self, sa: Point, sb: Point) -> f64 {
        self.distance_to_seg(sa, sb, |x, y| x * x + y * y)
    }

    ///Distance from segment end points to self
    pub fn distance_to_seg(&self, sa: Point, sb: Point, hypot_func: fn(f64, f64) -> f64) -> f64 {
        let (ax, ay) = (sa.x, sa.y);
        let (bx, by) = (sb.x, sb.y);
        let (px, py) = (self.x, self.y);
        let (dx, dy) = (bx - ax, by - ay);
        let isz_x = dx.feq(0.0);
        let isz_y = dy.feq(0.0);

        if isz_x && isz_y {
            //line with zero length
            hypot_func(px - ax, py - ay)
        } else {
            let u = (((px - ax) * dx) + ((py - ay) * dy)) / (dx * dx + dy * dy);
            let (c_ptx, c_pty) =
                if u < 0.0 {
                    (ax, ay)
                } else if u > 1.0 {
                    (bx, by)
                } else {
                    (ax + u * dx, ay + u * dy)
                };

            hypot_func(px - c_ptx, py - c_pty)
        }
    }
}

impl<T> From<(T, T)> for Point where T: NumCast + Copy {
    fn from(tup: (T, T)) -> Self {
        Point { x: num::cast(tup.0).unwrap(), y: num::cast(tup.1).unwrap() }
    }
}

impl<T> From<[T; 2]> for Point where T: NumCast + Copy {
    fn from(array: [T; 2]) -> Self {
        Point { x: num::cast(array[0]).unwrap(), y: num::cast(array[1]).unwrap() }
    }
}

impl From<&[f64]> for Point {
    fn from(slice: &[f64]) -> Self {
        Point { x: slice[0], y: slice[1] }
    }
}

impl From<&Vec<f64>> for Point {
    fn from(vec: &Vec<f64>) -> Self {
        Point { x: vec[0], y: vec[1] }
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

impl Coordinate for Point {
    type Scalar = f64;
    const DIM: usize = 2;
    ///Type Constructor : Generator over dimensions
    fn gen(value: impl Fn(usize) -> Self::Scalar) -> Self {
        Point {
            x: value(0),
            y: value(1),
        }
    }
    ///Value in ith dimension
    fn val(&self, i: usize) -> Self::Scalar {
        match i {
            0 => self.x,
            1 => self.y,
            _ => unreachable!(),
        }
    }
    ///Mutable value in ith dimension
    fn val_mut(&mut self, i: usize) -> &mut Self::Scalar {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable!(),
        }
    }
}

impl rstar::Point for Point {
    type Scalar = f64;
    const DIMENSIONS: usize = 2;

    ///Type Constructor : Generator over dimensions
    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
        Point::gen(generator)
    }

    ///Value in ith dimension
    fn nth(&self, index: usize) -> Self::Scalar {
        self.val(index)
    }

    ///Mutable value in ith dimension
    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        self.val_mut(index)
    }
}

impl Index<usize> for Point {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => unreachable!(),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.val_mut(i)
    }
}

#[macro_export]
macro_rules! pt {
    ($l :expr, $r:expr) => {
        Point{x : ($l) as f64, y : ($r) as f64}
    };
    [$l :expr, $r:expr] => {
        Point{x : ($l) as f64, y : ($r) as f64}
    };
}

#[cfg(test)]
mod tests;
